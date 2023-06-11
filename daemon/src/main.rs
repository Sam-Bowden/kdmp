use communication::{Request, Response};
use sink_thread_manager::SinkThreadManager;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

mod sink_thread_manager;

fn main() {
    //Open socket to receive requests from client.
    let socket = Path::new("/tmp/kdmp.sock");
    if socket.exists() {
        fs::remove_file(&socket).expect("Error deleting existing socket");
    }
    let listener = UnixListener::bind(&socket).expect("Error connecting to socket");

    let mut stm = SinkThreadManager::new();

    let mut current_song = String::new();

    //Loop over and process incoming requests from client
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let request: Request = bincode::deserialize_from(&stream).unwrap();

        match request {
            Request::Play(p) => {
                current_song = p.file_stem().unwrap().to_str().unwrap().to_string();
                stm.play_tracks(vec![p]);
            }
            Request::Stop => {
                if let Some(s) = &*stm.current_sink.lock().unwrap() {
                    s.stop();
                }
            }
            Request::Pause => {
                if let Some(s) = &*stm.current_sink.lock().unwrap() {
                    s.pause();
                }
            }
            Request::Resume => {
                if let Some(s) = &*stm.current_sink.lock().unwrap() {
                    s.play();
                }
            }
            Request::Next => {
                if let Some(s) = &*stm.current_sink.lock().unwrap() {
                    s.skip_one();
                }
            }
            Request::Status => (),
        }

        // Give enough time for action to apply
        thread::sleep(Duration::from_millis(50));

        let response = if let Some(s) = &*stm.current_sink.lock().unwrap() {
            if s.empty() {
                Response::Idle
            } else if s.is_paused() {
                Response::Paused(current_song.clone())
            } else {
                Response::Playing(current_song.clone())
            }
        } else {
            Response::Idle
        };

        let data = bincode::serialize(&response).unwrap();

        stream.write_all(&data).unwrap();
    }
}
