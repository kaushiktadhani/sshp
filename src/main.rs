use sshp::{get_ssh_config_path, parse_ssh_config, run_selector};
use std::process::Command;

fn main() {
    let config_path = match get_ssh_config_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if !config_path.exists() {
        eprintln!("SSH config file not found at: {}", config_path.display());
        eprintln!("Create one at ~/.ssh/config with Host entries");
        std::process::exit(1);
    }

    let hosts = match parse_ssh_config(&config_path) {
        Ok(hosts) => hosts,
        Err(e) => {
            eprintln!("Error reading SSH config: {}", e);
            std::process::exit(1);
        }
    };

    if hosts.is_empty() {
        eprintln!("No hosts found in SSH config file");
        std::process::exit(1);
    }

    match run_selector(hosts) {
        Ok(Some(host)) => {
            println!("Connecting to {}...", host);
            let status = Command::new("ssh")
                .arg(&host)
                .status()
                .expect("Failed to execute ssh command");

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
