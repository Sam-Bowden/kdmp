use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub track_directory: String,
}

impl Config {
    pub fn load() -> Config {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("kdmp").unwrap();

        let config_file_name = "kdmp.conf";

        let config_file_path = match xdg_dirs.find_config_file(config_file_name) {
            Some(cf) => cf,
            None => {
                xdg_dirs.place_config_file(config_file_name).expect("Error creating config file")
            }
        };

        let config_file = File::open(config_file_path).expect("Error opening config file");

        let config_reader = BufReader::new(config_file);
        
        serde_json::from_reader(config_reader).expect("Json config incorrectly formatted")
    }
}