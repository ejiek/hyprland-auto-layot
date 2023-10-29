use hyprland::data::Clients;
use hyprland::dispatch::*;
use hyprland::event_listener::State;
use hyprland::prelude::*;

use std::process::Command;
use std::{thread, time};

use log::{debug, error};

use eyre::Result;

use crate::config::Config;
use crate::helpers::*;

pub fn workspace_change_handler(state: &mut State, config: Config) -> Result<()> {
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
            debug!("No clients in the workspace, opening a placeholder");
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

    // TODO: replace with rotate_ws
    match get_monitor_orientation(&state.active_monitor, None) {
        Ok(Orientation::Vertical) => {
            debug!(
                "Setting vertical: ws {}, mon {}",
                &state.active_workspace, &state.active_monitor
            );
            Dispatch::call(config.vertical_layout.into()).unwrap();
        }
        Ok(Orientation::Horizontal) => {
            debug!(
                "Setting horizontal:  ws {}, mon {}",
                &state.active_workspace, &state.active_monitor
            );
            Dispatch::call(config.horizontal_layout.into()).unwrap();
        }
        Err(e) => {
            error!("Monitor not found: {:?}", e);
            // TODO: Handle error
        }
    };

    if window_placeholder.is_some() {
        debug!("Killing the placeholder");
        window_placeholder.unwrap().kill()?;
    }

    Ok(())
}
