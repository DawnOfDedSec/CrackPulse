mod functions;
mod helpers;

use tokio::time::Duration;
use std::env;

use functions::ssh;
use functions::winlogon;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    rdp_attack()
    // ssh_attack();
}

fn rdp_attack() {
    let hostname = String::from("192.168.2.12");
    let _ = winlogon::recon(hostname, Some(Vec::from([String::from("Rahee\\potency")])), None);
}

fn ssh_attack() {
    let hostname = String::from("127.0.0.1");
    let port = 22;

    let responses = ssh::recon(hostname, port, Duration::from_secs(5), None, None);
    for res in responses.iter() {
        println!(
            "username: {} | password: {} | result: {:?}",
            res.0.username,
            res.0.password,
            res.1
        );
    }
}
