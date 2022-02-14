use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use bincode;
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    command: String,
}

fn main() {
    let socket = Path::new("/tmp/kdmp.sock");

    if socket.exists() {
        fs::remove_file(&socket).expect("Error deleting existing socket");
    }

    let listener = UnixListener::bind(&socket).expect("Error connecting to socket");

    for stream in listener.incoming() {
        let request: Request = bincode::deserialize_from(&stream.unwrap()).unwrap();
        println!("{}", request.command);
    }
}
