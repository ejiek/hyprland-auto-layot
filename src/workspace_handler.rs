use hyprland::data:: Monitors;
use hyprland::dispatch::*;
use hyprland::event_listener::State;

use crate::helpers::*;

pub fn workspace_change_handler(state: &mut State, monitors: Monitors) {
    // TODO: Handle empty workspace
    match get_monitor_orientation(&state.active_monitor, &mut monitors.clone()){
        Ok(Orientation::Vertical) => {
            println!("Setting vertical orientation for {} at {}", &state.active_workspace, &state.active_monitor);
            Dispatch::call(DispatchType::OrientationTop).unwrap();
        },
        Ok(Orientation::Horizontal) => {
            println!("Setting horizontal orientation for {} at {}", &state.active_workspace, &state.active_monitor);
            Dispatch::call(DispatchType::OrientationCenter).unwrap();
        },
        Err(e) => {
            println!("Monitor not found: {:?}", e);
            // TODO: Handle error
        }

    }
}
