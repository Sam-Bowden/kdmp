use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub enum Request {
    Play(PathBuf),
    Stop,
    Pause,
    Resume,
    Next,
    Status,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Playing(String),
    Paused(String),
    Idle,
    DaemonNotOnline,
    DaemonCommunicationError,
}
