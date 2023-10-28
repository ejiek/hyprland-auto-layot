use eyre::{eyre, Result};
use hyprland::data::{Monitors, Transforms, Workspace};
use hyprland::dispatch::*;

use log::{debug, error};

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub fn rotate_ws(ws: Workspace, monitors: &mut Monitors) -> Result<()> {
    match get_monitor_orientation(&ws.monitor, monitors) {
        Ok(Orientation::Vertical) => {
            debug!("Setting vertical: ws {}, mon {}", &ws.id, &ws.monitor);
            Dispatch::call(DispatchType::OrientationTop)?;
        }
        Ok(Orientation::Horizontal) => {
            debug!("Setting horizontal: ws {}, mon {}", &ws.id, &ws.monitor);
            Dispatch::call(DispatchType::OrientationCenter)?;
        }
        Err(e) => {
            error!("Monitor not found: {:?}", e);
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
