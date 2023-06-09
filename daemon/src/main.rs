use request::Request;
use sink_thread_manager::SinkThreadManager;
use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;

mod request;
mod sink_thread_manager;

fn main() {
    //Open socket to receive requests from client.
    let socket = Path::new("/tmp/kdmp.sock");
    if socket.exists() {
        fs::remove_file(&socket).expect("Error deleting existing socket");
    }
    let listener = UnixListener::bind(&socket).expect("Error connecting to socket");

    let mut stm = SinkThreadManager::new();

    //Loop over and process incoming requests from client
    for stream in listener.incoming() {
        let request: Request = bincode::deserialize_from(&stream.unwrap()).unwrap();

        match request {
            Request::Play(p) => {
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
        }
    }
}
