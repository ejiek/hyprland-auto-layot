use eyre::{eyre, Result};
use hyprland::data::{Monitors, Transforms, Workspace};
use hyprland::dispatch::*;
use hyprland::prelude::*;

use log::{debug, error};

use crate::config::Config;

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub fn rotate_ws(ws: Workspace, config: &Config, monitors: Option<&Monitors>) -> Result<()> {
    match get_monitor_orientation(&ws.monitor, monitors) {
        Ok(Orientation::Vertical) => {
            debug!("Setting vertical: ws {}, mon {}", &ws.id, &ws.monitor);
            Dispatch::call(config.vertical_layout.into())?;
        }
        Ok(Orientation::Horizontal) => {
            debug!("Setting horizontal: ws {}, mon {}", &ws.id, &ws.monitor);
            Dispatch::call(config.horizontal_layout.into())?;
        }
        Err(e) => {
            error!("Monitor not found: {:?}", e);
        }
    }
    Ok(())
}

pub fn get_monitor_orientation(
    monitor_name: &str,
    monitors: Option<&Monitors>,
) -> Result<Orientation> {
    let monitors = match monitors {
        Some(m) => m.clone(),
        None => Monitors::get()?,
    };
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
