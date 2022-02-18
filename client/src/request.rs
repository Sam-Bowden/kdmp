use std::os::unix::net::UnixStream;
use bincode;
use serde::Serialize;
use std::path::PathBuf;
use crate::config;

#[derive(Serialize)]
pub enum Request {
    Begin(PathBuf),
    Stop,
    Pause,
    Resume,
}

impl Request {
    pub fn new(content: String, config: &config::Config) -> Request {
        let mut commands = content.split(" ");

        match commands.next().unwrap() {
            //Begin
            "b" => {
                let mut music_path = PathBuf::new();
                music_path.push(&config.music_directory);
                music_path.push(commands.next().expect("Missing song"));
                music_path.set_extension("mp3");
                Request::Begin(music_path)
            }
            //Stop
            "s" => Request::Stop,
            //Pause
            "p" => Request::Pause,
            //Resume
            "r" => Request::Resume, 
            _ => panic!("Invalid request"),
        }
    }
    pub fn send_request(&self) -> Result<(), &'static str> {
        let stream = match UnixStream::connect("/tmp/kdmp.sock") {
            Ok(s) => s,
            Err(_) => return Err("Failed to connect to daemon"),
        };

        match bincode::serialize_into(&stream, &self) {
            Ok(()) => Ok(()),
            Err(_) => Err("Failed to communicate to daemon"),
        }
    }
}
