use hyprland::dispatch::DispatchType;
use log::info;
use serde::Serialize;
use std::path::PathBuf;

#[derive(clap::ValueEnum, Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Center,
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub horizontal_layout: Layout,
    pub vertical_layout: Layout,
    pub placeholder_window: String,
    pub hyprland_conf: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Daemon,
    FireOnce(Option<PathBuf>),
}

impl Config {
    pub fn new(
        horizontal_layout: Layout,
        vertical_layout: Layout,
        placeholder_window: String,
        mode: Mode,
    ) -> Self {
        let hyprland_conf = match mode {
            Mode::FireOnce(conf_option) => match conf_option {
                Some(conf) => Some(conf),
                None => {
                    info!("No hyprland.conf path provided. Looking for one.");
                    let mut conf = dirs::config_dir().unwrap();
                    conf.push("hypr");
                    conf.push("hyprland.conf");
                    Some(conf)
                }
            },
            _ => None,
        };
        Self {
            horizontal_layout,
            vertical_layout,
            placeholder_window,
            hyprland_conf,
        }
    }
}

impl<'a> Into<DispatchType<'a>> for Layout {
    fn into(self) -> DispatchType<'a> {
        match self {
            Layout::Center => DispatchType::OrientationCenter,
            Layout::Left => DispatchType::OrientationLeft,
            Layout::Top => DispatchType::OrientationTop,
            Layout::Right => DispatchType::OrientationRight,
            Layout::Bottom => DispatchType::OrientationBottom,
        }
    }
}
