use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

/// Represents a parsed SSH host entry from the SSH config file.
///
/// This is a simplified representation containing the hostname and optional group.
/// Wildcard hosts (containing '*') are automatically excluded during parsing.
///
/// Groups are defined using `#group:groupname` comments in the SSH config file.
/// All hosts following a group comment belong to that group until a new group
/// comment is encountered.
#[derive(Debug, Clone, PartialEq)]
pub struct SshHost {
    /// The hostname as specified in the SSH config file
    pub name: String,
    /// The group this host belongs to (from #group:name comment)
    pub group: Option<String>,
}

/// Gets the path to the SSH config file.
///
/// Looks for the SSH config at `~/.ssh/config` on Unix-like systems (using HOME)
/// or `%USERPROFILE%/.ssh/config` on Windows (using USERPROFILE).
///
/// # Returns
///
/// Returns `Ok(PathBuf)` pointing to the SSH config file path, or `Err(String)`
/// if neither HOME nor USERPROFILE environment variables are set.
///
/// # Examples
///
/// ```
/// use sshp::get_ssh_config_path;
///
/// match get_ssh_config_path() {
///     Ok(path) => println!("SSH config at: {}", path.display()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn get_ssh_config_path() -> Result<PathBuf, String> {
    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home).join(".ssh").join("config"));
    }
    if let Ok(userprofile) = env::var("USERPROFILE") {
        return Ok(PathBuf::from(userprofile).join(".ssh").join("config"));
    }
    Err("Neither HOME nor USERPROFILE environment variable is set".to_string())
}

/// Parses an SSH config file and extracts all non-wildcard Host entries.
///
/// Reads the SSH config file line by line and extracts hostnames from lines
/// that start with "Host ". Wildcard hosts (containing '*') are automatically
/// excluded to prevent invalid selections.
///
/// # Arguments
///
/// * `path` - Path to the SSH config file to parse
///
/// # Returns
///
/// Returns `Ok(Vec<SshHost>)` containing all parsed hosts, or an `io::Error`
/// if the file cannot be read.
///
/// # Examples
///
/// ```no_run
/// use sshp::{get_ssh_config_path, parse_ssh_config};
///
/// let config_path = get_ssh_config_path().expect("Could not get config path");
/// let hosts = parse_ssh_config(&config_path).expect("Failed to parse config");
/// println!("Found {} hosts", hosts.len());
/// ```
pub fn parse_ssh_config(path: &PathBuf) -> io::Result<Vec<SshHost>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut hosts = Vec::new();
    let mut current_group: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Check for group comment (e.g., "#group:production", "#group:web", etc.)
        if trimmed.starts_with("#group:") {
            // Extract group name: "#group:production" -> "production"
            let group_name = trimmed[7..].trim().to_lowercase();
            if !group_name.is_empty() {
                current_group = Some(group_name);
            }
            continue;
        }

        // Regular comments (starting with # but not #group:) are ignored
        if trimmed.starts_with('#') {
            continue;
        }

        // Parse Host directive
        if trimmed.starts_with("Host ") && !trimmed.contains('*') {
            let host_name = trimmed[5..].trim().to_string();
            if !host_name.is_empty() {
                hosts.push(SshHost {
                    name: host_name,
                    group: current_group.clone(),
                });
            }
        }
    }

    Ok(hosts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_ssh_config_with_valid_hosts() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_valid");

        let config_content = r#"
Host server1
    HostName 192.168.1.1
    User admin

Host server2
    HostName example.com
    Port 2222

Host server3
    HostName test.local
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 3);
        assert_eq!(hosts[0].name, "server1");
        assert_eq!(hosts[0].group, None);
        assert_eq!(hosts[1].name, "server2");
        assert_eq!(hosts[1].group, None);
        assert_eq!(hosts[2].name, "server3");
        assert_eq!(hosts[2].group, None);

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_parse_ssh_config_excludes_wildcards() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_wildcards");

        let config_content = r#"
Host server1
    HostName 192.168.1.1

Host *
    User default

Host *.example.com
    Port 22

Host server2
    HostName example.com
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 2);
        assert_eq!(hosts[0].name, "server1");
        assert_eq!(hosts[0].group, None);
        assert_eq!(hosts[1].name, "server2");
        assert_eq!(hosts[1].group, None);

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_parse_ssh_config_empty_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_empty");

        fs::write(&config_path, "").unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 0);

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_parse_ssh_config_only_comments() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_comments");

        let config_content = r#"
# This is a comment
# Another comment
    # Indented comment
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 0);

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_parse_ssh_config_file_not_found() {
        let config_path = PathBuf::from("/nonexistent/path/to/ssh/config");

        let result = parse_ssh_config(&config_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_ssh_config_path_returns_result() {
        // This test verifies that the function returns a Result
        // The actual path will vary based on environment
        let result = get_ssh_config_path();

        // Should return Ok on systems with HOME or USERPROFILE set
        // In CI environments, this should always succeed
        assert!(result.is_ok());
    }

    #[test]
    fn test_ssh_host_struct() {
        let host = SshHost {
            name: "test-server".to_string(),
            group: None,
        };

        assert_eq!(host.name, "test-server");
        assert_eq!(host.group, None);
    }

    #[test]
    fn test_ssh_host_clone() {
        let host1 = SshHost {
            name: "server1".to_string(),
            group: Some("production".to_string()),
        };

        let host2 = host1.clone();

        assert_eq!(host1, host2);
    }

    #[test]
    fn test_group_comment_applies_to_subsequent_hosts() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_groups");

        let config_content = r#"
# This is a regular comment - should be ignored

#group:production
Host prod-web
    HostName 10.0.1.10

Host prod-db
    HostName 10.0.1.20

#group:staging
Host stage-web
    HostName 10.0.2.10
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 3);

        assert_eq!(hosts[0].name, "prod-web");
        assert_eq!(hosts[0].group, Some("production".to_string()));

        assert_eq!(hosts[1].name, "prod-db");
        assert_eq!(hosts[1].group, Some("production".to_string()));

        assert_eq!(hosts[2].name, "stage-web");
        assert_eq!(hosts[2].group, Some("staging".to_string()));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_regular_comments_are_ignored() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_regular_comments");

        let config_content = r#"
# This is a regular comment
## This is also a regular comment
# TODO: fix this server

Host server1
    HostName 10.0.1.1

#group:production
# Another regular comment
Host server2
    HostName 10.0.1.2
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 2);

        // server1 should have no group (regular comments don't set groups)
        assert_eq!(hosts[0].name, "server1");
        assert_eq!(hosts[0].group, None);

        // server2 should have production group
        assert_eq!(hosts[1].name, "server2");
        assert_eq!(hosts[1].group, Some("production".to_string()));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_hosts_before_group_comment_have_no_group() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_no_initial_group");

        let config_content = r#"
Host ungrouped-server
    HostName 192.168.1.1

#group:production
Host grouped-server
    HostName 10.0.1.1
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 2);

        assert_eq!(hosts[0].name, "ungrouped-server");
        assert_eq!(hosts[0].group, None);

        assert_eq!(hosts[1].name, "grouped-server");
        assert_eq!(hosts[1].group, Some("production".to_string()));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_empty_group_comment_is_ignored() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_empty_group");

        let config_content = r#"
#group:
Host server1
    HostName 10.0.1.1
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].name, "server1");
        assert_eq!(hosts[0].group, None);

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_group_whitespace_handling() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_group_whitespace");

        let config_content = r#"
#group:  production
Host server1
    HostName 10.0.1.1
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].name, "server1");
        assert_eq!(hosts[0].group, Some("production".to_string()));

        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn test_group_case_normalization() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_ssh_config_group_case");

        let config_content = r#"
#group:PRODUCTION
Host server1
    HostName 10.0.1.1

#group:StAgInG
Host server2
    HostName 10.0.2.1
"#;

        fs::write(&config_path, config_content).unwrap();

        let hosts = parse_ssh_config(&config_path).unwrap();

        assert_eq!(hosts.len(), 2);
        assert_eq!(hosts[0].group, Some("production".to_string()));
        assert_eq!(hosts[1].group, Some("staging".to_string()));

        fs::remove_file(config_path).unwrap();
    }
}
