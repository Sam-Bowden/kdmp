use std::os::unix::net::UnixStream;
use bincode;
use serde::{Serialize};
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Request {
    command: PathBuf,
}

impl Request {
    pub fn new(content: PathBuf) -> Request {
        Request { command: content }
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

