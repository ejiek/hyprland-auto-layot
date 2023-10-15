use eyre::{eyre, Result};
use hyprland::data::Workspaces;
use hyprland::prelude::*;

use crate::hyprland_conf::Config;

pub fn fire_once(verbose: bool, vertical_monitors: Vec<String>) -> Result<()> {
    if verbose {
        println!("Running in fireonce mode");
    }
    if vertical_monitors.is_empty() {
        return Err(eyre!("No vertical monitors found"));
    }

    let worksaces = Workspaces::get()?;
    println!("IPC Workspaces: {:?}", worksaces);

    let hyprland_config = Config::open_default()?;
    println!("Hyprland config: {:?}", hyprland_config);
    Ok(())
}
