use clap::Parser;
use eyre::Result;
use hyprland::data::Monitors;
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::prelude::*;
use serde::Serialize;

use log::{info, error};
use simple_logger::SimpleLogger;

mod hyprland_conf;

mod workspace_handler;
use workspace_handler::*;

mod fire_once;
use fire_once::*;

mod helpers;

#[derive(clap::ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgLayout {
    Center,
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Increase logging verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Daemonize the process (runs in the background)
    #[clap(short, long, conflicts_with = "fireonce")]
    daemon: bool,

    /// Go trough all workspaces once and then exit (default)
    #[clap(short, long)]
    fireonce: bool,

    /// What application to use as a windows placeholder to apply workspace
    /// orientation. Orientation doesn't apply to an empty workspace.
    #[clap(short, long, default_value = "alacritty")]
    placeholder_window: String,

    /// Horizontal orientation layout
    #[clap(long,
           value_name = "LAYOUT",
           value_enum,
           default_value_t = ArgLayout::Center)]
    horizontal_layout: ArgLayout,

    /// Vertical orientation layout
    #[clap(long,
           value_name = "LAYOUT",
           value_enum,
           default_value_t = ArgLayout::Top)]
    vertical_layout: ArgLayout,

    /// hyrpland.conf file path.
    /// If not provided the following paths are checked in order:
    /// $XDG_CONFIG_HOME/hyprland/hyprland.conf
    /// $HOME/.config/hyprland/hyprland.conf
    #[clap(long)]
    hyprland_conf: Option<String>,

}

fn main() -> Result<()> {
    let args = Args::parse();

    let logging_level = match args.verbose {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };
    SimpleLogger::new()
        .with_level(logging_level)
        .without_timestamps()
        .init()?;

    let monitors = Monitors::get()?;

    // TODO: Check if any vertical monitors are present

    if args.daemon {
        info!("Running in Daemon mode");
        let mut event_listener = EventListener::new();
        event_listener.add_workspace_change_handler(move |_id, state| {
            match workspace_change_handler(state, monitors.clone()) {
                Ok(_) => {}
                Err(e) => error!("Unable to handle workspace change event: {:?}", e),
            };
        });

        event_listener
            .start_listener()
            .map_err(|e| eyre::Report::new(e).wrap_err("Failed to start event listener"))?;
    } else {
        info!("Running in FireOnce mode");
        fire_once(monitors)?;
    };
    Ok(())
}
