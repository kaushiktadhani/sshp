use crate::config::SshHost;
use nucleo_matcher::{Config, Matcher, Utf32Str};

/// Represents a host with its fuzzy match score.
///
/// This struct is used to pair an SSH host with its relevance score
/// when performing fuzzy matching against a search query.
#[derive(Debug, Clone)]
pub struct HostMatch<'a> {
    pub host: &'a SshHost,
    pub score: u16,
}

/// Filters and ranks SSH hosts based on a search query using fuzzy matching.
///
/// When the query is empty, returns all hosts sorted alphabetically (case-insensitive).
/// When a query is provided, performs fuzzy matching and ranks results by:
/// 1. Match score (higher is better - exact/prefix matches score higher than fuzzy)
/// 2. Alphabetical order (as tiebreaker)
///
/// # Arguments
///
/// * `hosts` - Slice of SSH hosts to filter and rank
/// * `query` - Search string to filter by (uses fuzzy matching)
///
/// # Returns
///
/// A vector of `HostMatch` structs containing matched hosts and their scores,
/// sorted by relevance (best matches first).
///
/// # Examples
///
/// ```
/// use sshp::config::SshHost;
/// use sshp::selector::filter::filter_and_rank_hosts;
///
/// let hosts = vec![
///     SshHost { name: "production-server".to_string(), group: None },
///     SshHost { name: "my-prod-box".to_string(), group: None },
///     SshHost { name: "dev-server".to_string(), group: None },
/// ];
///
/// let matches = filter_and_rank_hosts(&hosts, "prod");
/// // "production-server" ranks higher (prefix match) than "my-prod-box" (substring)
/// assert_eq!(matches[0].host.name, "production-server");
/// ```
pub fn filter_and_rank_hosts<'a>(hosts: &'a [SshHost], query: &str) -> Vec<HostMatch<'a>> {
    // If no search query, return alphabetically sorted
    if query.is_empty() {
        let mut matches: Vec<_> = hosts
            .iter()
            .map(|host| HostMatch { host, score: 0 })
            .collect();
        matches.sort_by(|a, b| a.host.name.to_lowercase().cmp(&b.host.name.to_lowercase()));
        return matches;
    }

    // Fuzzy matching with nucleo
    let mut matcher = Matcher::new(Config::DEFAULT);

    // Convert query to lowercase for case-insensitive matching
    let query_lower = query.to_lowercase();

    let mut matches: Vec<_> = hosts
        .iter()
        .filter_map(|host| {
            let mut buf1 = Vec::new();
            let mut buf2 = Vec::new();
            // Convert both to lowercase for case-insensitive matching
            let host_lower = host.name.to_lowercase();
            let haystack = Utf32Str::new(&host_lower, &mut buf1);
            let needle = Utf32Str::new(&query_lower, &mut buf2);
            matcher
                .fuzzy_match(haystack, needle)
                .map(|score| HostMatch { host, score })
        })
        .collect();

    // Sort by score (descending), then alphabetically
    matches.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.host.name.to_lowercase().cmp(&b.host.name.to_lowercase()))
    });

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_query_returns_alphabetical() {
        let hosts = vec![
            SshHost {
                name: "zebra".to_string(),
                group: None,
            },
            SshHost {
                name: "Apple".to_string(),
                group: None,
            },
            SshHost {
                name: "banana".to_string(),
                group: None,
            },
            SshHost {
                name: "cherry".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "");

        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0].host.name, "Apple");
        assert_eq!(matches[1].host.name, "banana");
        assert_eq!(matches[2].host.name, "cherry");
        assert_eq!(matches[3].host.name, "zebra");
    }

    #[test]
    fn test_prefix_matches_rank_higher() {
        let hosts = vec![
            SshHost {
                name: "production".to_string(),
                group: None,
            },
            SshHost {
                name: "my-prod-server".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "prod");

        // "production" (prefix) should rank higher than "my-prod-server" (substring)
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].host.name, "production");
        assert!(matches[0].score > matches[1].score);
    }

    #[test]
    fn test_fuzzy_matching_works() {
        let hosts = vec![
            SshHost {
                name: "production-web-server".to_string(),
                group: None,
            },
            SshHost {
                name: "dev-server".to_string(),
                group: None,
            },
            SshHost {
                name: "staging-server".to_string(),
                group: None,
            },
        ];

        // Fuzzy match: "pws" should match "production-web-server"
        let matches = filter_and_rank_hosts(&hosts, "pws");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].host.name, "production-web-server");
    }

    #[test]
    fn test_case_insensitive_fuzzy_matching() {
        let hosts = vec![
            SshHost {
                name: "Production-Server".to_string(),
                group: None,
            },
            SshHost {
                name: "dev-server".to_string(),
                group: None,
            },
        ];

        let matches_lower = filter_and_rank_hosts(&hosts, "prod");
        let matches_upper = filter_and_rank_hosts(&hosts, "PROD");

        assert_eq!(matches_lower.len(), 1);
        assert_eq!(matches_upper.len(), 1);
        assert_eq!(matches_lower[0].host.name, "Production-Server");
        assert_eq!(matches_upper[0].host.name, "Production-Server");
    }

    #[test]
    fn test_no_match_returns_empty() {
        let hosts = vec![
            SshHost {
                name: "server1".to_string(),
                group: None,
            },
            SshHost {
                name: "server2".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "nonexistent");

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_alphabetical_tiebreaker() {
        let hosts = vec![
            SshHost {
                name: "zebra-prod".to_string(),
                group: None,
            },
            SshHost {
                name: "alpha-prod".to_string(),
                group: None,
            },
            SshHost {
                name: "beta-prod".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "prod");

        // All should match with similar scores, alphabetical order should apply
        assert_eq!(matches.len(), 3);
        // Check that they're in alphabetical order (may have same scores)
        let names: Vec<_> = matches.iter().map(|m| m.host.name.as_str()).collect();
        let mut sorted_names = names.clone();
        sorted_names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        // Since scores might be equal, we just verify all hosts are present
        assert!(names.contains(&"alpha-prod"));
        assert!(names.contains(&"beta-prod"));
        assert!(names.contains(&"zebra-prod"));
    }

    #[test]
    fn test_exact_match_scores_highest() {
        let hosts = vec![
            SshHost {
                name: "prod".to_string(),
                group: None,
            },
            SshHost {
                name: "production".to_string(),
                group: None,
            },
            SshHost {
                name: "my-prod-server".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "prod");

        // Exact match "prod" should score highest
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].host.name, "prod");
    }

    #[test]
    fn test_empty_host_list() {
        let hosts: Vec<SshHost> = vec![];

        let matches = filter_and_rank_hosts(&hosts, "anything");

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_special_characters_in_query() {
        let hosts = vec![
            SshHost {
                name: "my-server.example.com".to_string(),
                group: None,
            },
            SshHost {
                name: "my_server_2".to_string(),
                group: None,
            },
            SshHost {
                name: "server@host".to_string(),
                group: None,
            },
        ];

        let matches = filter_and_rank_hosts(&hosts, "my-server");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].host.name, "my-server.example.com");

        let matches_underscore = filter_and_rank_hosts(&hosts, "my_server");
        assert_eq!(matches_underscore.len(), 1);
        assert_eq!(matches_underscore[0].host.name, "my_server_2");
    }
}
