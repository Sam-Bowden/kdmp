use std::os::unix::net::UnixListener;
use std::path::Path;
use std::fs;
use playlist::PlayList;
use std::fs::File;
use std::io::BufReader;
use request::Request;
use sink_thread_manager::SinkThreadManager;

mod request;
mod playlist;
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
            Request::PlayTrack(p) => {
                stm.play_tracks(vec![p]);
            }
            Request::PlayList(p) => {
                let list_file = File::open(&p).unwrap();
                let list_reader = BufReader::new(list_file);
                let list: PlayList = serde_json::from_reader(list_reader).expect("JSON for PlayList incorrectly formatted");
                stm.play_tracks(list.tracks);
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
