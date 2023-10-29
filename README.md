# Hyprland auto layout

Optimizes your multi-monitor setup by automagically aligning the master layout orientation with the respective monitor orientation.
Started from a [Hyprland issue](https://github.com/hyprwm/Hyprland/issues/3174).

## Assumptions

- A multi-monitor setup
- Monitors have a different orientation
- Master layout utilization
- Monitor orientation is taken from transform value reported by IPC (`1`,`3`,`5`,`7` or `90`,`270` and their flipped versions)

## Roadmap

- [x] Add proper cli support with [clap](https://github.com/clap-rs/clap)
- [x] Implement Fire Once strategy to handle the initial setup
- [x] Configurable orientation (currently hardcoded)
- [x] Configurable or baked in placeholder window (currently hardcoded alacritty)
- [ ] Fix daemon mode ignoring switching to a ws active on another monitor
- [ ] Config file (proper xdg support)
- Documentation
  - [ ] Installation
  - [ ] Usage
  - [ ] Configuration
- [ ] Startup checks & warnings
- [ ] Figure out Special Workspaces
- [ ] Reduce placeholder usage in daemon mode (hyprland fix or state cache)

## Installation

To Be Done...

## How does it work

There are two strategies.

### 🔁 Daemonized

Listens for Hyprland IPC events.
On the `workspace` change event it checks a monitor+workspace pair and sets an orientation.
Currently, there is no way to figure out active orientation for a given workspace.
So orientation is updated every time.

**Pros**
- Adapts to workspace reassignments between monitors
- Accommodates newly added or removed monitors and workspaces
- Doesn't rely on parsing `hyrpland.conf`

**Cons**
- Runs continuously in the background
- Processes every Hyprland signal
- You might see flashing placeholder window more often (see [IPC missing workspace orientation](#IPC-missing-workspace-orientation))

**When to use**
Dynamic monitor and workspace setups where changes occur post-initialization.

`monitoradded` and `monitorremoved' are not handled yet but it should update a list of monitors.

### ➡️ Fire Once

On startup, this app goes through all workspaces and sets appropriate orientation.
Then returns to the default workspaces for each monitor.

**Pros:**
- Resource-efficient

**Cons:**
- Not adaptive (e.g., reassigning workspaces to different monitors, accommodating new monitors)
- Relies on a parsing `hyprland.conf`

**When to use**
Best suited for static workspace-to-monitor bindings

## Extras

### IPC missing workspaces

Hyprland doesn't report all workspaces through IPC (and `hyprctl`).
It reports only:

- workspaces with windows
- visible workspaces

Good thing is, that it stores orientation even for workspaces it doesn't currently report (it doesn't "forget" orientation).
This is why fire once strategy works.
But to find all the workspaces this tool needs to parse `hyprland.conf`.

Probably it's possible to fix this issue in Hyprland.

### IPC missing workspace orientation

There is no way to get current workspace orientation from IPC.
Because of it this app always assumes that current orientation is wrong and set the correct one.

### IPC ignoring orientation

It seems that it's not possible to set orientation to a workspace without any windows o.O
