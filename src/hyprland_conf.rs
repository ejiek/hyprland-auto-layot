use eyre::Result;
use std::fs::File;
use std::io::prelude::*;

use log::debug;

// Hyprland config
#[derive(Debug)]
pub struct Config {
    pub workspaces: Vec<Workspace>,
}

pub type Id = i32;

#[derive(Debug)]
pub struct Workspace {
    pub id: Id,
}

impl Config {
    pub fn open(path: std::path::PathBuf) -> Result<Self> {
        // Add file path to the error
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Config::parse(contents)
    }

    fn parse(raw_config: String) -> Result<Self> {
        let mut workspaces: Vec<Workspace> = Vec::new();
        let workspace_lines: Vec<&str> = raw_config
            .lines()
            .filter(|line| line.starts_with("workspace="))
            .map(|line| line.trim_start_matches("workspace="))
            .collect();

        workspace_lines
            .iter()
            .map(|line| *line.split(',').collect::<Vec<&str>>().first().unwrap())
            .filter(|id| id.parse::<Id>().is_ok())
            .for_each(|id| {
                workspaces.push(Workspace {
                    id: id.parse::<Id>().unwrap(),
                })
            });
        debug!("Defined workspaces: {:?}", workspace_lines);
        Ok(Config { workspaces })
    }
}
