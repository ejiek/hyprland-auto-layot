use eyre::Result;
use hyprland::data::{Monitor, Monitors, Transforms};
use hyprland::dispatch::*;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::event_listener::State;
use hyprland::prelude::*;

fn main() -> Result<()> {
    let monitors = Monitors::get()?;
    let vertical_monitors: Vec<String> = monitors
        .iter()
        .filter(|m| is_vertical(m))
        .map(|m| m.name.clone())
        .collect();
    println!("Vertical monitors: {:?}", vertical_monitors);

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |_id, state| {
        workspace_change_handler(state, vertical_monitors.clone());
    });

    event_listener
        .start_listener()
        .map_err(|e| eyre::Report::new(e).wrap_err("Failed to start event listener"))
}

fn workspace_change_handler(state: &mut State, vertical_monitors: Vec<String>) {
    match vertical_monitors
        .iter()
        .find(|m| m.eq(&&state.active_monitor))
    {
        Some(_) => {
            match Dispatch::call(DispatchType::OrientationTop) {
                Ok(_) => {
                    println!("Vertical Workspace");
                }
                Err(e) => {
                    println!("Hyprctl dispatch error: {:?}", e);
                }
            };
        }
        None => {
            match Dispatch::call(DispatchType::OrientationCenter) {
                Ok(_) => {
                    println!("Horizontal Workspace");
                }
                Err(e) => {
                    println!("Hyprctl dispatch error: {:?}", e);
                }
            };
        }
    };
}

fn is_vertical(monitor: &Monitor) -> bool {
    matches!(
        monitor.transform,
        Transforms::Normal90
            | Transforms::Normal270
            | Transforms::Flipped90
            | Transforms::Flipped270
    )
}
