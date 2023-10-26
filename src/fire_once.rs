use eyre::Result;
use hyprland::data::{Monitors, Workspace};
use hyprland::dispatch::*;
use hyprland::prelude::*;
use std::process::Command;

use crate::hyprland_conf::Config;
use crate::helpers::*;
use std::{thread, time};

// Save active workspaces for each monitor
// Save active workspace
//
// Loop through workspaces
// Move window to the workspace (it change active workspace)
// Set orientation for active workspace
//
// Kill the window
// Restore initial workspaces for each monitor
// Go to the initial workspace

pub fn fire_once(verbose: bool, monitors: Monitors) -> Result<()> {
    if verbose {
        println!("Running in fireonce mode");
    }

    // Monitors acquired on startup hold information about initial
    // active workspaces for each monitor

    // Non active workspaces are not available through the IPC
    // Therefore we need to get them from the config file
    let hyprland_config = Config::open_default()?;
    if verbose {
      println!("Hyprland config: {:?}", hyprland_config);
    }

    let initial_ws = Workspace::get_active()?;
    if verbose {
        println!("Initial active workspace: {:?}", initial_ws);
    }

    let mut workspaces = hyprland_config.workspaces.into_iter();

    let first_ws = workspaces.next().unwrap();
    Dispatch::call(DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(first_ws.id),
    ))?;

    // TODO: make placeholder configurable
    // even better: draw a window from this app
    // even better: fix hyprland not setting orientation on an empty ws
    let mut window_placeholder = Command::new("alacritty")
        .spawn()
        .expect("Failet to start alacritty");

    // Wait for the window to appear
    thread::sleep(time::Duration::from_millis(500));

    for w in workspaces {
        // Move window to the next workspace
        Dispatch::call(DispatchType::MoveToWorkspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(w.id),
            Some(WindowIdentifier::ProcessId(window_placeholder.id())),
        ))?;

        let current_ws = Workspace::get_active()?;
        rotate_ws(current_ws, &mut monitors.clone())?;
    }

    window_placeholder.kill()?;

    for m in monitors {
        Dispatch::call(DispatchType::Workspace(
            hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(m.active_workspace.id),
        ))?;
    }

    Dispatch::call(DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(initial_ws.id),
    ))?;

    Ok(())
}
