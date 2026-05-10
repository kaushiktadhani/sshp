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
    use sshp::selector::filter::filter_and_rank_hosts;

    let filtered = filter_and_rank_hosts(&hosts, "production");
    assert_eq!(filtered.len(), 2);

    let filtered_web = filter_and_rank_hosts(&hosts, "web");
    assert_eq!(filtered_web.len(), 2);

    let filtered_specific = filter_and_rank_hosts(&hosts, "staging-web");
    assert_eq!(filtered_specific.len(), 1);
    assert_eq!(filtered_specific[0].host.name, "staging-web-01");

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
        group: None,
    };

    let host2 = SshHost {
        name: "server1".to_string(),
        group: None,
    };

    let host3 = SshHost {
        name: "server2".to_string(),
        group: None,
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
    use sshp::selector::filter::filter_and_rank_hosts;

    let filtered = filter_and_rank_hosts(&hosts, "server-0");
    assert_eq!(filtered.len(), 50); // All match "server-0"

    let filtered_specific = filter_and_rank_hosts(&hosts, "server-025");
    assert_eq!(filtered_specific.len(), 1);
    assert_eq!(filtered_specific[0].host.name, "server-025");

    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_group_functionality() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_groups");

    let config_content = r#"
# Regular comment - should be ignored

#group:production
Host prod-web-01
    HostName 10.0.1.10
    User admin

Host prod-db-01
    HostName 10.0.1.20
    User postgres

#group:staging
Host stage-web-01
    HostName 10.0.2.10
    User admin

Host stage-db-01
    HostName 10.0.2.20
    User postgres

# Personal servers (ungrouped - no #group: comment to reset)
# Note: dev-local will inherit staging group from above
# To have no group, it would need to appear before any #group: comment

#group:development
Host dev-local
    HostName localhost
    Port 2222
"#;

    fs::write(&config_path, config_content).unwrap();

    // Parse the config
    let hosts = parse_ssh_config(&config_path).expect("Failed to parse SSH config");

    // Verify parsing with groups
    assert_eq!(hosts.len(), 5);

    // Check production hosts have production group
    let prod_web = hosts.iter().find(|h| h.name == "prod-web-01").unwrap();
    assert_eq!(prod_web.group, Some("production".to_string()));

    let prod_db = hosts.iter().find(|h| h.name == "prod-db-01").unwrap();
    assert_eq!(prod_db.group, Some("production".to_string()));

    // Check staging hosts have staging group
    let stage_web = hosts.iter().find(|h| h.name == "stage-web-01").unwrap();
    assert_eq!(stage_web.group, Some("staging".to_string()));

    let stage_db = hosts.iter().find(|h| h.name == "stage-db-01").unwrap();
    assert_eq!(stage_db.group, Some("staging".to_string()));

    // Check development host has development group
    let dev_local = hosts.iter().find(|h| h.name == "dev-local").unwrap();
    assert_eq!(dev_local.group, Some("development".to_string()));

    // Test that groups are parsed correctly
    // (Group filtering via @group syntax has been removed; groups are now only used for display)
    use sshp::selector::filter::filter_and_rank_hosts;

    // Test regular search still works across all hosts
    let web_hosts = filter_and_rank_hosts(&hosts, "web");
    assert_eq!(web_hosts.len(), 2); // prod-web-01 and stage-web-01

    // Clean up
    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_fuzzy_matching() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_fuzzy");

    let config_content = r#"
Host production-web-server
    HostName 10.0.1.10

Host production-database-server
    HostName 10.0.1.20

Host staging-web-server
    HostName 10.0.2.10
"#;

    fs::write(&config_path, config_content).unwrap();

    let hosts = parse_ssh_config(&config_path).expect("Failed to parse SSH config");
    use sshp::selector::filter::filter_and_rank_hosts;

    // Test fuzzy matching: "pws" should match "production-web-server"
    let matches = filter_and_rank_hosts(&hosts, "pws");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].host.name, "production-web-server");

    // Test fuzzy matching: "pdbs" should match "production-database-server"
    let matches = filter_and_rank_hosts(&hosts, "pdbs");
    assert!(matches.len() >= 1);
    assert_eq!(matches[0].host.name, "production-database-server");

    // Test that prefix matches rank higher than fuzzy matches
    let matches = filter_and_rank_hosts(&hosts, "prod");
    assert_eq!(matches.len(), 2);
    // Both production servers should match, with prefix scoring higher
    assert!(matches[0].host.name.starts_with("production"));

    // Clean up
    fs::remove_file(config_path).unwrap();
}

#[test]
fn test_alphabetical_sorting() {
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("integration_test_sorting");

    let config_content = r#"
Host zebra-server
    HostName 10.0.1.1

Host alpha-server
    HostName 10.0.1.2

Host beta-server
    HostName 10.0.1.3
"#;

    fs::write(&config_path, config_content).unwrap();

    let hosts = parse_ssh_config(&config_path).expect("Failed to parse SSH config");
    use sshp::selector::filter::filter_and_rank_hosts;

    // With empty query, hosts should be sorted alphabetically
    let matches = filter_and_rank_hosts(&hosts, "");
    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0].host.name, "alpha-server");
    assert_eq!(matches[1].host.name, "beta-server");
    assert_eq!(matches[2].host.name, "zebra-server");

    // Clean up
    fs::remove_file(config_path).unwrap();
}
