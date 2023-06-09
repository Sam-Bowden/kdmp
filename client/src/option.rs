use std::path::{Path, PathBuf};

pub enum Option {
    Command {
        description: String,
        keywords: Vec<String>,
    },
    Track {
        name: String,
        path: PathBuf,
    },
}

impl Option {
    pub fn new_command(description: &str) -> Self {
        Self::Command {
            description: description.to_string(),
            keywords: vec![],
        }
    }

    pub fn new_track(path: &Path) -> Self {
        Self::Track {
            name: path.file_stem().unwrap().to_str().unwrap().to_string(),
            path: PathBuf::from(path),
        }
    }

    pub fn display(&self) -> &str {
        match self {
            Self::Command { description, .. } => description,
            Self::Track { name, .. } => name,
        }
    }

    pub fn keyword(mut self, kw: &str) -> Self {
        match &mut self {
            Self::Command {
                description: _,
                keywords,
            } => keywords.push(kw.to_string()),
            Self::Track { .. } => panic!("Can only add keyword to command option"),
        }
        return self;
    }

    pub fn get_path(&self) -> &PathBuf {
        if let Self::Track { name: _n, path: p } = self {
            p
        } else {
            panic!("Can only retrieve path from track option")
        }
    }

    pub fn starts_with(&self, value: &str) -> bool {
        match self {
            Self::Command {
                description: _,
                keywords,
            } => keywords.iter().any(|x| x.starts_with(value)),
            Self::Track { name, .. } => name.starts_with(value),
        }
    }
}
