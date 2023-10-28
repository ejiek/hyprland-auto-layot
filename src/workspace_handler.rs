use hyprland::data::{Clients, Monitors};
use hyprland::dispatch::*;
use hyprland::event_listener::State;
use hyprland::prelude::*;

use std::process::Command;
use std::{thread, time};

use eyre::Result;

use crate::helpers::*;

pub fn workspace_change_handler(state: &mut State, monitors: Monitors) -> Result<()> {
    // TODO: Handle empty workspace

    // Check if there are any clients in the workspace
    // If there are no clients, open a place holder
    // Check if the current monitor is vertical or horizontal
    // Rotate the monitor accordingly
    // Kill the place holder

    // Get client list
    let clients = Clients::get()?;
    let current_clients = clients
        .iter()
        .filter(|c| match state.active_workspace.clone() {
            hyprland::shared::WorkspaceType::Regular(name) => c.workspace.name == name,
            hyprland::shared::WorkspaceType::Special(whoknowswhat) => match whoknowswhat {
                Some(name) => c.workspace.name == name,
                None => false,
            },
        })
        .collect::<Vec<_>>();

    let window_placeholder = match current_clients.is_empty() {
        true => {
            // TODO: make placeholder configurable
            // even better: draw a window from this app
            // even better: fix hyprland not setting orientation on an empty ws
            println!("No clients in the workspace, opening placeholder");
            let placeholder = Some(
                Command::new("alacritty")
                    .spawn()
                    .expect("Failet to start alacritty"),
            );
            // Wait for the window to appear
            // TODO: find a way to do it without hardcoded sleep value
            thread::sleep(time::Duration::from_millis(500));
            placeholder
        }
        false => None,
    };

    match get_monitor_orientation(&state.active_monitor, &mut monitors.clone()) {
        Ok(Orientation::Vertical) => {
            println!(
                "Setting vertical orientation for {} at {}",
                &state.active_workspace, &state.active_monitor
            );
            Dispatch::call(DispatchType::OrientationTop).unwrap();
        }
        Ok(Orientation::Horizontal) => {
            println!(
                "Setting horizontal orientation for {} at {}",
                &state.active_workspace, &state.active_monitor
            );
            Dispatch::call(DispatchType::OrientationCenter).unwrap();
        }
        Err(e) => {
            println!("Monitor not found: {:?}", e);
            // TODO: Handle error
        }
    };

    if window_placeholder.is_some() {
        window_placeholder.unwrap().kill()?;
    }

    Ok(())
}
