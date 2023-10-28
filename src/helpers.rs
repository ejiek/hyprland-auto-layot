use eyre::{eyre, Result};
use hyprland::data::{Monitors, Transforms, Workspace};
use hyprland::dispatch::*;

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub fn rotate_ws(ws: Workspace, monitors: &mut Monitors) -> Result<()> {
    match get_monitor_orientation(&ws.monitor, monitors) {
        Ok(Orientation::Vertical) => {
            println!(
                "Setting vertical orientation for {} at {}",
                &ws.id, &ws.monitor
            );
            Dispatch::call(DispatchType::OrientationTop)?;
        }
        Ok(Orientation::Horizontal) => {
            println!(
                "Setting horizontal orientation for {} at {}",
                &ws.id, &ws.monitor
            );
            Dispatch::call(DispatchType::OrientationCenter)?;
        }
        Err(e) => {
            println!("Monitor not found: {:?}", e);
        }
    }
    Ok(())
}

pub fn get_monitor_orientation(monitor_name: &str, monitors: &mut Monitors) -> Result<Orientation> {
    for m in monitors {
        if m.name.eq(&monitor_name) {
            if matches!(
                m.transform,
                Transforms::Normal90
                    | Transforms::Normal270
                    | Transforms::Flipped90
                    | Transforms::Flipped270
            ) {
                return Ok(Orientation::Vertical);
            } else {
                return Ok(Orientation::Horizontal);
            }
        }
    }
    Err(eyre!("Monitor not found"))
}
