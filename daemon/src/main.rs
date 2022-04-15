use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use bincode;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use request::Request;
use playlist::PlayList;

mod request;
mod playlist;

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
            Request::PlayTrack(p) => {
                sink.clear();
                let track_file = File::open(&p).unwrap();
                let source = Decoder::new(BufReader::new(track_file)).unwrap();
                sink.append(source);
                if sink.len() == 2 {
                    sink.skip_one();
                }
                sink.play();
            }
            Request::PlayList(p) => {
                sink.clear();
                let list_file = File::open(&p).unwrap();
                let list_reader = BufReader::new(list_file);
                let list: PlayList = serde_json::from_reader(list_reader).expect("JSON for PlayList incorrectly formatted");
                for track in &list.tracks {
                    let track_file = File::open(&track).unwrap();
                    let source = Decoder::new(BufReader::new(track_file)).unwrap();
                    sink.append(source);
                }
                if sink.len() > list.tracks.len() {
                    sink.skip_one();
                }
                sink.play();
            }
            Request::Stop => {
                sink.clear();
            }
            Request::Pause => sink.pause(),
            Request::Resume => sink.play(),
            Request::Next => {
                if sink.len() > 1 {
                    sink.skip_one();
                } else {
                    sink.pause();
                }
            }
        }
    }
}