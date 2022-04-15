use std::fs;
use crate::{request::Request, config::Config};
use std::path::PathBuf;
use std::ffi::OsStr;

pub struct Command {
	pub text: String,
	pub options: Vec<String>,
	all_tracks: Vec<String>,
	all_lists: Vec<String>,
	config: Config,
}

impl Command {
	pub fn new() -> Command {
		//Load tracks and lists
		let config = Config::load();
		let mut all_tracks = Vec::new();
		let mut all_lists = Vec::new();
		let dir = fs::read_dir(&config.music_directory).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
            	if path.extension() == Some(OsStr::new("mp3")) {
            		let path = path.file_stem().unwrap();
                	all_tracks.push(path.to_string_lossy().to_string());
            	}
                
                if path.extension() == Some(OsStr::new("json")) {
                	let path = path.file_stem().unwrap();
                	all_lists.push(path.to_string_lossy().to_string());
                }
            }
        }

		Command { 
			text: String::new(),
			options: Vec::new(),
			all_tracks: all_tracks,
			all_lists: all_lists,
			config: config,
		}
	}

	pub fn execute(&mut self) -> Result<(), &'static str> {
		let command = self.text.clone();
		let mut command_components = command.split(" ");

		match command_components.next() {
			Some("pt") => self.play_track(),
			Some("pl") => self.play_list(),
			Some("s") => Request::Stop.send(),
        	Some("p") => Request::Pause.send(),
        	Some("r") => Request::Resume.send(), 
        	Some("n") => Request::Next.send(),
			_ => Err("Invalid command"),
		}
	}

	pub fn update_options(&mut self) {
		self.options.clear();

		let command = self.text.clone();
		let mut command_components = command.split(" ");

		match command_components.next() {
			Some("pt") => if let Some(n) = command_components.next() { self.track_options(n) },
			Some("pl") => if let Some(n) = command_components.next() { self.list_options(n) },
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

	fn list_options(&mut self, name: &str) {
		for list in &self.all_lists {
		   	if list.len() >= name.len() {
		   		if list[..name.len()].eq(name) { self.options.push(list.to_string()); }
		   	}
    	}
	}

	fn play_track(&self) -> Result<(), &'static str> {
		if self.options.len() > 0 {
			let mut track_path = PathBuf::new();
			track_path.push(&self.config.music_directory);
            track_path.push(&self.options[0]);
            track_path.set_extension("mp3");
			Request::PlayTrack(track_path).send()
		} else {
			Err("No option selected")
		}
	}

	fn play_list(&self) -> Result<(), &'static str> {
		if self.options.len() > 0 {
			let mut list_path = PathBuf::new();
			list_path.push(&self.config.music_directory);
            list_path.push(&self.options[0]);
            list_path.set_extension("json");
			Request::PlayList(list_path).send()
		} else {
			Err("No option selected")
		}
	}
}