use connection::connect_to_backdoor;

mod command;
mod connection;

fn main() {
    let mut stream = connect_to_backdoor("localhost:1337").unwrap();
    command::run_command(String::from("dir"), &mut stream).unwrap();
    println!("Disconnecting from backdoor.");
    // Connection is properly closed here when stream's owner goes out of scope.
}
