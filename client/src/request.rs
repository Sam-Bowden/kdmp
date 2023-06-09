use crate::error_menu::ErrorMenu;
use crate::event::Event;
use bincode;
use serde::Serialize;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

#[derive(Serialize)]
pub enum Request {
    Play(PathBuf),
    Stop,
    Pause,
    Resume,
    Next,
}

impl Request {
    pub fn send(&self) -> Event {
        let Ok(stream) = UnixStream::connect("/tmp/kdmp.sock") else {
            return Event::OpenErrorMenu(ErrorMenu::FailedToConnectToDaemon);
        };

        match bincode::serialize_into(&stream, &self) {
            Ok(()) => Event::None,
            Err(_) => Event::OpenErrorMenu(ErrorMenu::FailedToConnectToDaemon),
        }
    }
}
