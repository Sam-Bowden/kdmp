use std::os::unix::net::UnixStream;
use bincode;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub enum Request {
    PlayTrack(PathBuf),
    PlayList(PathBuf),
    Stop,
    Pause,
    Resume,
    Next,
}

impl Request {
    pub fn send(&self) -> Result<(), &'static str> {
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