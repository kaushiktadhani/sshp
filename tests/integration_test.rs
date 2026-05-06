use sshp::{get_ssh_config_path, parse_ssh_config, SshHost};
use std::fs;

#[test]
fn test_full_workflow_parse_and_filter() {
    // Create a temporary SSH config file
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_ssh_config");

    let config_content = r#"
Host production-web-01
    HostName 10.0.1.10
    User admin

Host production-db-01
    HostName 10.0.1.20
    User postgres

Host staging-web-01
    HostName 10.0.2.10
    User admin

Host development-local
    HostName localhost
    Port 2222

Host *
    User default
"#;

    fs::write(&config_path, config_content).unwrap();

    // Parse the config
    let hosts = parse_ssh_config(&config_path).expect("Failed to parse SSH config");

    // Verify parsing
    assert_eq!(hosts.len(), 4);
    assert!(hosts.iter().any(|h| h.name == "production-web-01"));
    assert!(hosts.iter().any(|h| h.name == "production-db-01"));
    assert!(hosts.iter().any(|h| h.name == "staging-web-01"));
    assert!(hosts.iter().any(|h| h.name == "development-local"));

    // Test filtering with the selector's filter function
    use sshp::selector::filter::filter_hosts;

    let filtered = filter_hosts(&hosts, "production");
    assert_eq!(filtered.len(), 2);

    let filtered_web = filter_hosts(&hosts, "web");
    assert_eq!(filtered_web.len(), 2);

    let filtered_specific = filter_hosts(&hosts, "staging-web");
    assert_eq!(filtered_specific.len(), 1);
    assert_eq!(filtered_specific[0].name, "staging-web-01");

    // Clean up
    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_ssh_config_path_resolution() {
    // Test that we can get a config path
    let result = get_ssh_config_path();

    // Should succeed on any system with HOME or USERPROFILE
    assert!(result.is_ok());

    let path = result.unwrap();

    // Path should end with .ssh/config
    assert!(
        path.to_string_lossy().ends_with(".ssh/config")
            || path.to_string_lossy().ends_with(".ssh\\config")
    );
}

#[test]
fn test_parse_handles_various_formats() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_formats");

    // Test various SSH config formats
    let config_content = r#"
# Comment at the start

Host simple
    HostName example.com

Host   with-extra-spaces
    HostName example.com

Host multiple-options
    HostName example.com
    User admin
    Port 2222
    IdentityFile ~/.ssh/id_rsa

# Inline comment
Host after-comment
    HostName example.com
"#;

    fs::write(&config_path, config_content).unwrap();

    let hosts = parse_ssh_config(&config_path).unwrap();

    assert_eq!(hosts.len(), 4);
    assert!(hosts.iter().any(|h| h.name == "simple"));
    assert!(hosts.iter().any(|h| h.name == "with-extra-spaces"));
    assert!(hosts.iter().any(|h| h.name == "multiple-options"));
    assert!(hosts.iter().any(|h| h.name == "after-comment"));

    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_ssh_host_equality() {
    let host1 = SshHost {
        name: "server1".to_string(),
    };

    let host2 = SshHost {
        name: "server1".to_string(),
    };

    let host3 = SshHost {
        name: "server2".to_string(),
    };

    assert_eq!(host1, host2);
    assert_ne!(host1, host3);
}

#[test]
fn test_large_config_file() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_large");

    let mut config_content = String::new();

    // Generate 50 hosts to test performance without edge cases
    for i in 1..=50 {
        config_content.push_str(&format!(
            "Host server-{:03}\n    HostName 10.0.{}.{}\n    User admin\n\n",
            i,
            i / 255 + 1,
            i % 255 + 1
        ));
    }

    fs::write(&config_path, &config_content).unwrap();

    let hosts = parse_ssh_config(&config_path).unwrap();

    assert_eq!(hosts.len(), 50);
    assert_eq!(hosts[0].name, "server-001");
    assert_eq!(hosts[49].name, "server-050");

    // Test filtering performance on large list
    use sshp::selector::filter::filter_hosts;

    let filtered = filter_hosts(&hosts, "server-0");
    assert_eq!(filtered.len(), 50); // All match "server-0"

    let filtered_specific = filter_hosts(&hosts, "server-025");
    assert_eq!(filtered_specific.len(), 1);
    assert_eq!(filtered_specific[0].name, "server-025");

    fs::remove_file(config_path).unwrap();
}
