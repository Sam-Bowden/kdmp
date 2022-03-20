use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub enum Request {
    PlayTrack(PathBuf),
    Stop,
    Pause,
    Resume,
}