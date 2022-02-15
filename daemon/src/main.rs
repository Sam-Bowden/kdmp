use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use bincode;
use serde::Deserialize;
use std::thread;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

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
        thread::spawn(move || {
            play_music(request.command);
        });
    }
}

fn play_music(request: String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open(&request).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
