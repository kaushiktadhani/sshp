use crate::config::SshHost;
use crate::selector::filter::filter_hosts;
use crate::ui::render_ui;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io;

/// Runs the interactive SSH host selector in the terminal.
///
/// Presents a full-screen TUI with filtering capabilities for selecting
/// an SSH host. The user can type to filter hosts, navigate with arrow keys,
/// and press Enter to select or Esc/Ctrl+C to cancel.
///
/// # Arguments
///
/// * `hosts` - Vector of SSH hosts to select from
///
/// # Returns
///
/// * `Ok(Some(String))` - User selected a host (returns hostname)
/// * `Ok(None)` - User cancelled the selection (Esc or Ctrl+C)
/// * `Err(io::Error)` - Terminal/IO error occurred
///
/// # Examples
///
/// ```no_run
/// use sshp::{get_ssh_config_path, parse_ssh_config, run_selector};
///
/// let config_path = get_ssh_config_path().expect("Could not get config path");
/// let hosts = parse_ssh_config(&config_path).expect("Failed to parse");
///
/// match run_selector(hosts) {
///     Ok(Some(host)) => println!("Connecting to {}...", host),
///     Ok(None) => println!("Cancelled"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn run_selector(hosts: Vec<SshHost>) -> io::Result<Option<String>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut selected = 0;
    let mut query = String::new();
    let mut filtered_hosts = filter_hosts(&hosts, &query);

    render_ui(&filtered_hosts, selected, &query)?;

    // Drain any pending input events (e.g. the Enter keypress that launched this command)
    while event::poll(std::time::Duration::from_millis(50))? {
        let _ = event::read()?;
    }

    let result = loop {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    break None;
                }
                KeyCode::Esc => {
                    break None;
                }
                KeyCode::Up if selected > 0 => {
                    selected = selected.saturating_sub(1);
                }
                KeyCode::Down if selected < filtered_hosts.len().saturating_sub(1) => {
                    selected += 1;
                }
                KeyCode::Enter if !filtered_hosts.is_empty() => {
                    break Some(filtered_hosts[selected].name.clone());
                }
                KeyCode::Backspace => {
                    query.pop();
                    filtered_hosts = filter_hosts(&hosts, &query);
                    selected = selected.min(filtered_hosts.len().saturating_sub(1));
                }
                KeyCode::Char(c) => {
                    query.push(c);
                    filtered_hosts = filter_hosts(&hosts, &query);
                    selected = 0;
                }
                _ => {}
            }
            render_ui(&filtered_hosts, selected, &query)?;
        }
    };

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(result)
}
