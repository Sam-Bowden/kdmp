use std::fs;
use crate::{request::Request, config::Config};
use std::path::PathBuf;

pub struct Command {
	pub text: String,
	pub options: Vec<String>,
	all_tracks: Vec<String>,
	config: Config,
}

impl Command {
	pub fn new() -> Command {
		//Load tracks
		let config = Config::load();
		let mut all_tracks = Vec::new();
		let dir = fs::read_dir(&config.track_directory).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let path = path.file_stem().unwrap();
                all_tracks.push(path.to_string_lossy().to_string());
            }
        }

		Command { 
			text: String::new(),
			options: Vec::new(),
			all_tracks: all_tracks,
			config: config,
		}
	}

	pub fn execute(&mut self) -> Result<(), &'static str> {
		let command = self.text.clone();
		let mut command_components = command.split(" ");

		match command_components.next() {
			Some("pt") => self.play_track(),
			Some("s") => Request::Stop.send(),
        	Some("p") => Request::Pause.send(),
        	Some("r") => Request::Resume.send(), 
			_ => Err("Invalid command"),
		}
	}

	pub fn update_options(&mut self) {
		self.options.clear();

		let command = self.text.clone();
		let mut command_components = command.split(" ");

		match command_components.next() {
			Some("pt") => if let Some(n) = command_components.next() { self.track_options(n) },
			_ => (),
		};
	}

	fn track_options(&mut self, name: &str) {
		for track in &self.all_tracks {
		   	if track.len() >= name.len() {
		   		if track[..name.len()].eq(name) { self.options.push(track.to_string()); }
		   	}
    	}
	}

	fn play_track(&self) -> Result<(), &'static str> {
		if self.options.len() > 0 {
			let mut music_path = PathBuf::new();
			music_path.push(&self.config.track_directory);
            music_path.push(&self.options[0]);
            music_path.set_extension("mp3");
			Request::PlayTrack(music_path).send()
		} else {
			Err("No option selected")
		}
	}
}