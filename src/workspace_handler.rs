use hyprland::dispatch::*;
use hyprland::event_listener::State;

pub fn workspace_change_handler(state: &mut State, vertical_monitors: Vec<String>) {
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
