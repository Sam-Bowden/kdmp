use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub enum Request {
    Play(PathBuf),
    Stop,
    Pause,
    Resume,
    Next,
}
