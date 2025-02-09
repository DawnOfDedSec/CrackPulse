use std::io;

use crate::{models::CliError, utils::LogManager};

pub fn banner() {
    println!("
   █████████                               █████      ███████████             ████                  
  ███░░░░░███                             ░░███      ░░███░░░░░███           ░░███                  
 ███     ░░░  ████████   ██████    ██████  ░███ █████ ░███    ░███ █████ ████ ░███   █████   ██████ 
░███         ░░███░░███ ░░░░░███  ███░░███ ░███░░███  ░██████████ ░░███ ░███  ░███  ███░░   ███░░███
░███          ░███ ░░░   ███████ ░███ ░░░  ░██████░   ░███░░░░░░   ░███ ░███  ░███ ░░█████ ░███████ 
░░███     ███ ░███      ███░░███ ░███  ███ ░███░░███  ░███         ░███ ░███  ░███  ░░░░███░███░░░  
 ░░█████████  █████    ░░████████░░██████  ████ █████ █████        ░░████████ █████ ██████ ░░██████ 
  ░░░░░░░░░  ░░░░░      ░░░░░░░░  ░░░░░░  ░░░░ ░░░░░ ░░░░░          ░░░░░░░░ ░░░░░ ░░░░░░   ░░░░░░  
                                                                                                    
                                                                                                                                                                                                  
                                 CrackPulse - A Multi-threaded  Bruteforce Tool")
}

pub fn get(question: &str) -> &str {
    println!("[Crack-Pulse] (Q) -> {}", question);

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => {
            LogManager::eprint(Some("Input"), CliError::CantReadUserInput(err));
            std::process::exit(0)
        }
    }

    // Trim the newline character from the input
    input.trim()
}

pub fn start() {
    let method = match get("Enter Attack Method: \n1) Windows Login\n2) SSH Login") {
        "1" => winlogon_attack(),
        "2" => ssh_attack(),
        input => {
            LogManager::eprint(
                Some("Input"),
                CliError::InvalidUserInput(&input.to_string()),
            );
            std::process::exit(0)
        }
    };

    return ();
}

pub fn winlogon_attack() {
    let mut targets: Vec<String> = Vec::new();

    let hostname = get("Enter Windows Hostname (ex: example.com): ");
    let username = get("Enter Windows Username (ex: admin): ");
    let password = get("Enter Windows Password (ex: password123): ");
}

pub fn ssh_attack() {
    let mut targets: Vec<String> = Vec::new();

    let hostname = get("Enter Windows Hostname (ex: example.com): ");
    let username = get("Enter Windows Username (ex: admin): ");
    let password = get("Enter Windows Password (ex: password123): ");
}

pub fn targets() {
    return match get("Enter Target Fetching Method: \n1) User Input\n2) From CSV") {
        "1" => get("Enter Multiple Targets (ex: 172.168.5.1,192.168.1.2): "),
        "2" => {
            let location = get("Enter CSV File Path (ex: targets.csv): ");

            // Read targets from CSV file
            let targets = match read_csv(input) {
                Ok(targets) => targets,
                Err(err) => {
                    LogManager::eprint(Some("CSV"), err);
                    std::process::exit(0)
                }
            };
            targets
        }
        input => {
            LogManager::eprint(
                Some("Input"),
                CliError::InvalidUserInput(&input.to_string()),
            );
            std::process::exit(0)
        }
    };

    return ();
}
