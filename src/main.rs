mod functions;
mod helpers;

use tokio::time::Duration;
use std::env;

use functions::ssh;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let hostname = String::from("127.0.0.1");
    let port = 22;

    let responses = ssh::recon(hostname, port,Duration::from_secs(5));
    for res in responses.iter() {
        println!(
            "username: {} | password: {} | result: {:?}",
            res.0.username,
            res.0.password,
            res.1
        );
    }
}
