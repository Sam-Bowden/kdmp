use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use bincode;
use serde::Deserialize;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize)]
enum Request {
    Begin(PathBuf),
    Stop,
    Pause,
    Resume,
}

fn main() {
    let socket = Path::new("/tmp/kdmp.sock");

    if socket.exists() {
        fs::remove_file(&socket).expect("Error deleting existing socket");
    }

    let listener = UnixListener::bind(&socket).expect("Error connecting to socket");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for stream in listener.incoming() {
        let request: Request = bincode::deserialize_from(&stream.unwrap()).unwrap();

        match request {
            Request::Begin(p) => {
                if sink.len() > 0 {
                    sink.skip_one();
                }
                let file = File::open(&p).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);
            }
            Request::Stop => {
                if sink.len() > 0 {
                    sink.skip_one();
                }
            }
            Request::Pause => sink.pause(),
            Request::Resume => sink.play(),
        }
    }
}

