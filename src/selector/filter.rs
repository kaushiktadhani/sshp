use crate::config::SshHost;

/// Filters a list of SSH hosts based on a case-insensitive search query.
///
/// Returns all hosts if the query is empty, otherwise returns only hosts
/// whose names contain the query string (case-insensitive match).
///
/// # Arguments
///
/// * `hosts` - Slice of SSH hosts to filter
/// * `query` - Search string to filter by (case-insensitive)
///
/// # Returns
///
/// A vector of references to hosts that match the query
///
/// # Examples
///
/// ```
/// use sshp::config::SshHost;
/// use sshp::selector::filter::filter_hosts;
///
/// let hosts = vec![
///     SshHost { name: "production-server".to_string() },
///     SshHost { name: "dev-server".to_string() },
///     SshHost { name: "staging-server".to_string() },
/// ];
///
/// let filtered = filter_hosts(&hosts, "prod");
/// assert_eq!(filtered.len(), 1);
/// assert_eq!(filtered[0].name, "production-server");
///
/// let filtered_case = filter_hosts(&hosts, "PROD");
/// assert_eq!(filtered_case.len(), 1);
/// ```
pub fn filter_hosts<'a>(hosts: &'a [SshHost], query: &str) -> Vec<&'a SshHost> {
    if query.is_empty() {
        hosts.iter().collect()
    } else {
        hosts
            .iter()
            .filter(|host| host.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_hosts_empty_query_returns_all() {
        let hosts = vec![
            SshHost {
                name: "server1".to_string(),
            },
            SshHost {
                name: "server2".to_string(),
            },
            SshHost {
                name: "server3".to_string(),
            },
        ];

        let filtered = filter_hosts(&hosts, "");

        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_filter_hosts_case_insensitive() {
        let hosts = vec![
            SshHost {
                name: "Production-Server".to_string(),
            },
            SshHost {
                name: "dev-server".to_string(),
            },
            SshHost {
                name: "STAGING-SERVER".to_string(),
            },
        ];

        let filtered = filter_hosts(&hosts, "prod");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Production-Server");

        let filtered_upper = filter_hosts(&hosts, "PROD");
        assert_eq!(filtered_upper.len(), 1);
        assert_eq!(filtered_upper[0].name, "Production-Server");
    }

    #[test]
    fn test_filter_hosts_partial_match() {
        let hosts = vec![
            SshHost {
                name: "web-server-01".to_string(),
            },
            SshHost {
                name: "web-server-02".to_string(),
            },
            SshHost {
                name: "db-server-01".to_string(),
            },
        ];

        let filtered = filter_hosts(&hosts, "web");
        assert_eq!(filtered.len(), 2);

        let filtered_server = filter_hosts(&hosts, "server");
        assert_eq!(filtered_server.len(), 3);
    }

    #[test]
    fn test_filter_hosts_no_match() {
        let hosts = vec![
            SshHost {
                name: "server1".to_string(),
            },
            SshHost {
                name: "server2".to_string(),
            },
        ];

        let filtered = filter_hosts(&hosts, "nonexistent");

        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_hosts_special_characters() {
        let hosts = vec![
            SshHost {
                name: "my-server.example.com".to_string(),
            },
            SshHost {
                name: "my_server_2".to_string(),
            },
            SshHost {
                name: "server@host".to_string(),
            },
        ];

        let filtered = filter_hosts(&hosts, "my-server");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "my-server.example.com");

        let filtered_underscore = filter_hosts(&hosts, "my_server");
        assert_eq!(filtered_underscore.len(), 1);
        assert_eq!(filtered_underscore[0].name, "my_server_2");
    }

    #[test]
    fn test_filter_hosts_empty_list() {
        let hosts: Vec<SshHost> = vec![];

        let filtered = filter_hosts(&hosts, "anything");

        assert_eq!(filtered.len(), 0);
    }
}
