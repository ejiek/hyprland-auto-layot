use eyre::{eyre, Result};
use hyprland::data::{Workspace, Workspaces};
use hyprland::dispatch::*;
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

    let initial_workspace = Workspace::get_active()?;
    println!("Active workspace: {:?}", initial_workspace.id);

    for w in hyprland_config.workspaces {
        Dispatch::call(DispatchType::Workspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(w.id),
        ))?;
        let current_ws = Workspace::get_active()?;
        if vertical_monitors.contains(&current_ws.monitor) {
            println!(
                "Setting vertical orientation for {} at {}",
                &current_ws.id, &current_ws.monitor
            );
            Dispatch::call(DispatchType::OrientationTop)?;
        } else {
            println!(
                "Setting vertical orientation for {} at {}",
                &current_ws.id, &current_ws.monitor
            );
            Dispatch::call(DispatchType::OrientationCenter)?;
        }
    }

    Dispatch::call(DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(initial_workspace.id),
    ))?;

    Ok(())
}
