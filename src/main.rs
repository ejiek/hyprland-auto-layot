use clap::Parser;
use eyre::Result;
use hyprland::event_listener::EventListenerMutable as EventListener;
use std::path::PathBuf;

use log::{debug, error, info};
use simple_logger::SimpleLogger;

mod hyprland_conf;

mod workspace_handler;
use workspace_handler::*;

mod fire_once;
use fire_once::*;

mod helpers;

mod config;
use config::{Config, Layout, Mode};

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
           default_value_t = Layout::Center)]
    horizontal_layout: Layout,

    /// Vertical orientation layout
    #[clap(long,
           value_name = "LAYOUT",
           value_enum,
           default_value_t = Layout::Top)]
    vertical_layout: Layout,

    /// hyrpland.conf file path.
    /// If not provided the following paths are checked in order:
    /// $XDG_CONFIG_HOME/hyprland/hyprland.conf
    /// $HOME/.config/hyprland/hyprland.conf
    #[clap(long)]
    hyprland_conf: Option<PathBuf>,
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

    let mode = match args.fireonce {
        true => Mode::FireOnce(args.hyprland_conf),
        false => Mode::Daemon,
    };

    let config = Config::new(
        args.horizontal_layout,
        args.vertical_layout,
        args.placeholder_window,
        mode,
    );

    debug!("Using config: {:?}", config);

    // TODO: Check if any vertical monitors are present

    if args.daemon {
        info!("Running in Daemon mode");
        let mut event_listener = EventListener::new();
        event_listener.add_workspace_change_handler(move |_id, state| {
            match workspace_change_handler(state, config.clone()) {
                Ok(_) => {}
                Err(e) => error!("Unable to handle workspace change event: {:?}", e),
            };
        });

        event_listener
            .start_listener()
            .map_err(|e| eyre::Report::new(e).wrap_err("Failed to start event listener"))?;
    } else {
        info!("Running in FireOnce mode");
        fire_once(config)?;
    };
    Ok(())
}
