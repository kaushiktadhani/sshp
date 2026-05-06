use crate::config::SshHost;
use crate::ui::highlight::render_host_with_highlight;
use crossterm::{
    cursor, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

/// Renders the interactive SSH host selector UI to the terminal.
///
/// Displays a full-screen terminal UI with:
/// - Title bar
/// - Scrollable list of hosts with the selected host highlighted
/// - Search bar showing the current query
/// - Help text with keyboard shortcuts
///
/// The UI automatically handles scrolling for large lists and highlights
/// matching text in host names based on the search query.
///
/// # Arguments
///
/// * `filtered_hosts` - Slice of host references to display (already filtered)
/// * `selected` - Index of the currently selected host
/// * `query` - Current search query string
///
/// # Returns
///
/// Returns `Ok(())` on success or an `io::Error` if rendering fails.
pub fn render_ui(filtered_hosts: &[&SshHost], selected: usize, query: &str) -> io::Result<()> {
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

    // Calculate visible range - list grows from bottom
    let max_visible = (term_height as usize).saturating_sub(6);
    let start_idx = if selected >= max_visible {
        selected.saturating_sub(max_visible - 1)
    } else {
        0
    };
    let end_idx = (start_idx + max_visible).min(filtered_hosts.len());

    let visible_hosts: Vec<_> = filtered_hosts[start_idx..end_idx].iter().collect();
    let num_visible = visible_hosts.len();

    // Calculate starting line for hosts (to position them from bottom up)
    let hosts_end_line = term_height.saturating_sub(3); // Leave space for search and help
    let hosts_start_line = hosts_end_line.saturating_sub(num_visible as u16);

    // Render hosts from bottom
    for (i, host) in visible_hosts.iter().enumerate() {
        let actual_idx = start_idx + i;
        let line_pos = hosts_start_line + i as u16;

        queue!(stdout, cursor::MoveTo(0, line_pos))?;

        if actual_idx == selected {
            queue!(
                stdout,
                SetBackgroundColor(Color::Blue),
                SetForegroundColor(Color::White),
                Print("  ▶ ")
            )?;
            render_host_with_highlight(&mut stdout, &host.name, query)?;
            queue!(stdout, ResetColor)?;
        } else {
            queue!(stdout, SetForegroundColor(Color::White), Print("    "))?;
            render_host_with_highlight(&mut stdout, &host.name, query)?;
            queue!(stdout, ResetColor)?;
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
        Print(" to navigate, "),
        SetForegroundColor(Color::Green),
        Print("type"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to search, "),
        SetForegroundColor(Color::Green),
        Print("Enter"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to connect, "),
        SetForegroundColor(Color::Red),
        Print("Esc/Ctrl+C"),
        SetForegroundColor(Color::DarkGrey),
        Print(" to quit"),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}
