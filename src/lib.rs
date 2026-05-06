//! # sshp - Interactive SSH Host Selector
//!
//! A blazingly fast, interactive SSH host selector inspired by fzf.
//!
//! This library provides functionality to parse SSH config files and present
//! an interactive terminal UI for selecting and connecting to SSH hosts.
//!
//! ## Example
//!
//! ```no_run
//! use sshp::{get_ssh_config_path, parse_ssh_config, run_selector};
//!
//! fn main() {
//!     let config_path = get_ssh_config_path().expect("Could not get config path");
//!     let hosts = parse_ssh_config(&config_path).expect("Failed to parse config");
//!
//!     if let Ok(Some(selected_host)) = run_selector(hosts) {
//!         println!("Selected: {}", selected_host);
//!     }
//! }
//! ```

pub mod config;
pub mod selector;
pub mod ui;

pub use config::{get_ssh_config_path, parse_ssh_config, SshHost};
pub use selector::run_selector;
