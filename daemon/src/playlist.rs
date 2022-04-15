use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct PlayList {
    pub tracks: Vec<PathBuf>,
}