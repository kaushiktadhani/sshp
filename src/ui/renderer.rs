use crate::selector::filter::HostMatch;
use crate::ui::highlight::render_host_with_highlight;
use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::collections::BTreeMap;
use std::io::{self, Write};

/// Represents a group of hosts for rendering
struct GroupedHost<'a> {
    group_name: Option<String>, // None for "Other" section
    hosts: Vec<&'a HostMatch<'a>>,
    show_header: bool, // Whether to render the group header
}

/// Represents an item to render (either a group header or a host)
#[derive(Clone)]
enum RenderItem<'a> {
    GroupHeader(Option<String>), // None = "Other" section
    Host {
        match_item: &'a HostMatch<'a>,
        is_selected: bool,
    },
}

/// Groups hosts by their group field for rendering
fn group_matches<'a>(matches: &'a [HostMatch], has_any_groups: bool) -> Vec<GroupedHost<'a>> {
    // Separate hosts with groups from hosts without groups
    let mut grouped_hosts: BTreeMap<String, Vec<&HostMatch>> = BTreeMap::new();
    let mut ungrouped_hosts: Vec<&HostMatch> = Vec::new();

    for match_item in matches {
        match &match_item.host.group {
            Some(group) => {
                grouped_hosts
                    .entry(group.clone())
                    .or_insert_with(Vec::new)
                    .push(match_item);
            }
            None => {
                ungrouped_hosts.push(match_item);
            }
        }
    }

    // Build result: grouped sections first (alphabetically), then "Other" at bottom
    let mut result: Vec<GroupedHost> = grouped_hosts
        .into_iter()
        .map(|(group_name, hosts)| GroupedHost {
            group_name: Some(group_name),
            hosts,
            show_header: true, // Always show headers for named groups
        })
        .collect();

    // Add ungrouped hosts at the end under "Other" header (only if there are any)
    if !ungrouped_hosts.is_empty() {
        result.push(GroupedHost {
            group_name: None, // Will be rendered as "Other"
            hosts: ungrouped_hosts,
            show_header: has_any_groups, // Only show "Other" if groups exist
        });
    }

    result
}

/// Calculates which items should be visible on screen, accounting for scrolling and group headers
fn calculate_visible_items<'a>(
    grouped: &'a [GroupedHost],
    selected_flat_index: usize,
    max_visible_lines: usize,
) -> Vec<RenderItem<'a>> {
    // Build groups with their items (header + hosts)
    let mut groups_with_items: Vec<Vec<RenderItem>> = Vec::new();
    let mut flat_host_index = 0;

    for group in grouped {
        let mut group_items = Vec::new();

        // Add group header only if show_header is true
        if group.show_header {
            group_items.push(RenderItem::GroupHeader(group.group_name.clone()));
        }

        // Add hosts
        for match_item in &group.hosts {
            let is_selected = flat_host_index == selected_flat_index;

            group_items.push(RenderItem::Host {
                match_item,
                is_selected,
            });

            flat_host_index += 1;
        }

        // Reverse the group so hosts come before header
        // When rendered upward: last host at bottom, header at top of group
        group_items.reverse();

        groups_with_items.push(group_items);
    }

    // Reverse groups for bottom-up rendering (last group alphabetically appears first at bottom)
    groups_with_items.reverse();

    // Flatten groups back into a single list
    let all_items: Vec<RenderItem> = groups_with_items.into_iter().flatten().collect();

    // Find the selected item index in the reversed list
    let selected_item_index = all_items
        .iter()
        .position(|item| matches!(item, RenderItem::Host { is_selected: true, .. }))
        .unwrap_or(0);

    // Calculate visible window
    let total_items = all_items.len();
    if total_items <= max_visible_lines {
        // Everything fits, show all
        return all_items;
    }

    // Calculate scroll window to keep selected item visible
    let start_idx = if selected_item_index >= max_visible_lines {
        selected_item_index.saturating_sub(max_visible_lines - 1)
    } else {
        0
    };
    let end_idx = (start_idx + max_visible_lines).min(total_items);

    // Try to include group header if the first visible item is a host
    // and there's a group header immediately above it
    let adjusted_start = if start_idx > 0 {
        if matches!(all_items[start_idx], RenderItem::Host { .. }) {
            if matches!(all_items[start_idx - 1], RenderItem::GroupHeader(_)) {
                start_idx - 1
            } else {
                start_idx
            }
        } else {
            start_idx
        }
    } else {
        start_idx
    };

    let adjusted_end = if adjusted_start < start_idx {
        end_idx.saturating_sub(1).min(total_items)
    } else {
        end_idx
    };

    all_items[adjusted_start..adjusted_end].to_vec()
}

/// Renders the interactive SSH host selector UI to the terminal.
///
/// Displays a full-screen terminal UI with:
/// - Title bar
/// - Scrollable list of hosts grouped by their group field
/// - Search bar showing the current query
/// - Help text with keyboard shortcuts
///
/// Groups are displayed as section headers with hosts listed underneath.
/// Hosts without groups are shown under an "Other" header at the bottom,
/// but only if at least one group exists in the full SSH config.
///
/// The UI automatically handles scrolling for large lists and highlights
/// matching text in host names based on the search query.
///
/// # Arguments
///
/// * `filtered_matches` - Slice of HostMatch structs to display (already filtered and ranked)
/// * `selected` - Index of the currently selected host
/// * `query` - Current search query string
/// * `has_any_groups` - Whether any groups exist in the full SSH config
///
/// # Returns
///
/// Returns `Ok(())` on success or an `io::Error` if rendering fails.
pub fn render_ui(
    filtered_matches: &[HostMatch],
    selected: usize,
    query: &str,
    has_any_groups: bool,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    let (term_width, term_height) = terminal::size()?;

    queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Title
    queue!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("SSH Host Selector"),
        ResetColor,
        cursor::MoveToNextLine(1)
    )?;

    // Separator
    queue!(
        stdout,
        SetForegroundColor(Color::Blue),
        Print("─".repeat(term_width as usize)),
        ResetColor,
        cursor::MoveToNextLine(1)
    )?;

    // Group the matches
    let grouped = group_matches(filtered_matches, has_any_groups);

    // Calculate which items are visible, accounting for group headers
    let max_visible = (term_height as usize).saturating_sub(6);
    let visible_items = calculate_visible_items(&grouped, selected, max_visible);

    // Calculate starting line for bottom-up rendering (just above search bar)
    let search_bar_line = term_height - 2;
    let mut current_line = search_bar_line.saturating_sub(1);

    // Clear the content area (between separator and search bar)
    let hosts_start_line = 2; // After title and separator
    for line in hosts_start_line..search_bar_line {
        queue!(stdout, cursor::MoveTo(0, line), Print(" ".repeat(term_width as usize)))?;
    }

    // Render hosts with group headers from bottom to top
    // visible_items has: reversed groups, reversed hosts within groups, header at end of each group
    // We iterate forward, rendering upward - first item (last host of last group) appears at bottom
    for item in visible_items.iter() {
        match item {
            RenderItem::GroupHeader(group_name_opt) => {
                queue!(stdout, cursor::MoveTo(0, current_line))?;

                // Display group name or "Other" for ungrouped hosts
                let display_name = group_name_opt.as_deref().unwrap_or("Other");

                queue!(
                    stdout,
                    SetForegroundColor(Color::Cyan),
                    Print(display_name),
                    ResetColor
                )?;
                current_line = current_line.saturating_sub(1);
            }
            RenderItem::Host {
                match_item,
                is_selected,
            } => {
                queue!(stdout, cursor::MoveTo(0, current_line))?;

                if *is_selected {
                    queue!(
                        stdout,
                        SetBackgroundColor(Color::Blue),
                        SetForegroundColor(Color::White),
                        Print("  ▶ ")
                    )?;
                    render_host_with_highlight(&mut stdout, &match_item.host.name, query)?;
                    queue!(stdout, ResetColor)?;
                } else {
                    queue!(stdout, SetForegroundColor(Color::White), Print("    "))?;
                    render_host_with_highlight(&mut stdout, &match_item.host.name, query)?;
                    queue!(stdout, ResetColor)?;
                }
                current_line = current_line.saturating_sub(1);
            }
        }
    }

    // Search bar at bottom (above help text)
    queue!(stdout, cursor::MoveTo(0, term_height - 2))?;
    queue!(stdout, SetForegroundColor(Color::Cyan), Print("Search: "))?;
    if query.is_empty() {
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("___"),
            ResetColor
        )?;
    } else {
        queue!(
            stdout,
            ResetColor,
            SetForegroundColor(Color::Yellow),
            Print(query),
            Print("_"),
            ResetColor
        )?;
    }

    // Help text at bottom
    queue!(stdout, cursor::MoveTo(0, term_height - 1))?;
    queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("Use "),
        SetForegroundColor(Color::Green),
        Print("↑/↓"),
        SetForegroundColor(Color::DarkGrey),
        Print(", "),
        SetForegroundColor(Color::Green),
        Print("type"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to search, "),
        SetForegroundColor(Color::Green),
        Print("Enter"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to connect, "),
        SetForegroundColor(Color::Red),
        Print("Esc"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to quit"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SshHost;

    #[test]
    fn test_group_matches_hides_other_header_when_no_groups() {
        // Create test hosts without any groups
        let host1 = SshHost {
            name: "server1".to_string(),
            group: None,
        };
        let host2 = SshHost {
            name: "server2".to_string(),
            group: None,
        };

        let matches = vec![
            HostMatch {
                host: &host1,
                score: 100,
            },
            HostMatch {
                host: &host2,
                score: 100,
            },
        ];

        // Test with has_any_groups = false (no groups in config)
        let grouped = group_matches(&matches, false);
        assert_eq!(grouped.len(), 1);
        assert!(!grouped[0].show_header); // Should NOT show header
        assert_eq!(grouped[0].hosts.len(), 2);
    }

    #[test]
    fn test_group_matches_shows_other_header_when_groups_exist() {
        // Create test hosts with some grouped and some ungrouped
        let host1 = SshHost {
            name: "prod-server".to_string(),
            group: Some("production".to_string()),
        };
        let host2 = SshHost {
            name: "ungrouped-server".to_string(),
            group: None,
        };

        let matches = vec![
            HostMatch {
                host: &host1,
                score: 100,
            },
            HostMatch {
                host: &host2,
                score: 100,
            },
        ];

        // Test with has_any_groups = true (groups exist in config)
        let grouped = group_matches(&matches, true);
        assert_eq!(grouped.len(), 2);

        // First group should be "production" with header shown
        assert_eq!(grouped[0].group_name, Some("production".to_string()));
        assert!(grouped[0].show_header);

        // Second group should be "Other" with header shown
        assert_eq!(grouped[1].group_name, None); // None = "Other"
        assert!(grouped[1].show_header); // Should show "Other" header
    }

    #[test]
    fn test_group_matches_all_grouped_no_other_section() {
        // Create test hosts where all have groups
        let host1 = SshHost {
            name: "prod-server".to_string(),
            group: Some("production".to_string()),
        };
        let host2 = SshHost {
            name: "dev-server".to_string(),
            group: Some("development".to_string()),
        };

        let matches = vec![
            HostMatch {
                host: &host1,
                score: 100,
            },
            HostMatch {
                host: &host2,
                score: 100,
            },
        ];

        // Test with has_any_groups = true
        let grouped = group_matches(&matches, true);
        assert_eq!(grouped.len(), 2);

        // Should only have the two named groups, no "Other" section
        assert!(grouped[0].group_name.is_some());
        assert!(grouped[1].group_name.is_some());
    }
}
