use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use std::io::Read;

fn main() {
    let socket = Path::new("/tmp/kdmp.sock");

    if socket.exists() {
        fs::remove_file(&socket).expect("Error deleting existing socket");
    }

    let listener = UnixListener::bind(&socket).expect("Error connecting to socket");

    for stream in listener.incoming() {
        let mut response = String::new();
        stream.unwrap().read_to_string(&mut response).unwrap();
        println!("{}", response);
    }
}
