use serde::Deserialize;

#[derive(Deserialize)]
enum Request {
    Begin(PathBuf),
    Stop,
    Pause,
    Resume,
}