use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

/// Represents a parsed SSH host entry from the SSH config file.
///
/// This is a simplified representation containing only the hostname.
/// Wildcard hosts (containing '*') are automatically excluded during parsing.
#[derive(Debug, Clone, PartialEq)]
pub struct SshHost {
    /// The hostname as specified in the SSH config file
    pub name: String,
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

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.starts_with("Host ") && !trimmed.contains('*') {
            let host_name = trimmed[5..].trim().to_string();
            if !host_name.is_empty() {
                hosts.push(SshHost { name: host_name });
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
        assert_eq!(hosts[1].name, "server2");
        assert_eq!(hosts[2].name, "server3");

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
        assert_eq!(hosts[1].name, "server2");

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
        };

        assert_eq!(host.name, "test-server");
    }

    #[test]
    fn test_ssh_host_clone() {
        let host1 = SshHost {
            name: "server1".to_string(),
        };

        let host2 = host1.clone();

        assert_eq!(host1, host2);
    }
}
