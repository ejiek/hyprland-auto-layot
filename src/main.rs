use clap::Parser;
use eyre::Result;
use hyprland::data::{Monitor, Monitors, Transforms};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::prelude::*;

mod hyprland_conf;

mod workspace_handler;
use workspace_handler::*;

mod fire_once;
use fire_once::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // increased level of logging
    #[clap(short, long)]
    verbose: bool,

    /// Daemonize the process (runs in the background)
    #[clap(short, long, conflicts_with = "fireonce")]
    daemon: bool,

    /// Goes trough all workspaces once and then exits (default)
    #[clap(short, long)]
    fireonce: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let monitors = Monitors::get()?;
    let vertical_monitors: Vec<String> = monitors
        .iter()
        .filter(|m| is_vertical(m))
        .map(|m| m.name.clone())
        .collect();

    if args.verbose {
        println!("Vertical Monitors: {:?}", vertical_monitors);
    };

    if args.daemon {
        if args.verbose {
            println!("Running in daemon mode");
        }
        let mut event_listener = EventListener::new();
        event_listener.add_workspace_change_handler(move |_id, state| {
            workspace_change_handler(state, vertical_monitors.clone());
        });

        event_listener
            .start_listener()
            .map_err(|e| eyre::Report::new(e).wrap_err("Failed to start event listener"))?;
    } else {
        fire_once(args.verbose, vertical_monitors)?;
    };
    Ok(())
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
