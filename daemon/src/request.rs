use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub enum Request {
    PlayTrack(PathBuf),
    PlayList(PathBuf),
    Stop,
    Pause,
    Resume,
    Next,
}