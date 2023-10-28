# Hyprland auto layout

Optimizes your multi-monitor setup by automagically aligning the master layout orientation with the respective monitor orientation.
Started from a [Hyprland issue](https://github.com/hyprwm/Hyprland/issues/3174).

## Assumptions

- A multi-monitor setup
- Monitors have a different orientation
- Master layout utilization
- Monitor orientation is taken from transform value reported by IPC (`1`,`3`,`5`,`7` or `90`,`270` and their flipped versions)
- ⏳ *Temporary*
  - Central orientation for horizontal
  - Top orientation for vertical

## Roadmap

- [x] Add proper cli support with [clap](https://github.com/clap-rs/clap)
- [x] Implement Fire Once strategy to handle the initial setup
- [ ] Transition from hyprland-rs to reduce size (this tool needs just a couple of dispatchers and a little bit of unix socket parsing)
- [ ] Startup checks & warnings for Fire once
- [ ] Make orientation configurable (currently hardcoded)
- [ ] Documentation (installation, usage, configuration)
- [ ] Figure out Special Workspaces

## Installation

To Be Done...

## How does it work

Then there are two strategies.

### Daemonized

Listens for Hyprland IPC events.
On the `workspace` change event it checks a monitor+workspace pair and sets an orientation.
Currently, there is no way to figure out active orientation for a given workspace.
So orientation is updated every time.

**Pros**
- Adapts to workspace reassignments between monitors
- Accommodates newly added or removed monitors and workspaces.

**Cons**
- Runs continuously in the background
- Processes every Hyprland signal

**When to use**
Dynamic monitor and workspace setups where changes occur post-initialization.

`monitoradded` and `monitorremoved' are not handled yet but it should update a list of monitors.

### Fire Once

On startup, this app goes through all workspaces and sets appropriate orientation.
Then returns to the default workspaces for each monitor.

**Pros:**
- Resource-efficient

**Cons:**
- Not adaptive (e.g., reassigning workspaces to different monitors, accommodating new monitors)
- Initialization takes some time and may look weird (dancing cursor)

**When to use**
Best suited for static workspace-to-monitor bindings

## Extras

### IPC missing information

Hyprland doesn't report all workspaces through IPC (and `hyprctl`).
It reports only:

- workspaces with windows
- visible workspaces

Good thing is, that it stores orientation even for workspaces it doesn't currently report (it doesn't "forget" orientation).
This is why fire once strategy works.
But to find all the workspaces this tool needs to parse `hyprland.conf`.

Probably it's possible to fix this issue in Hyprland.

### IPC ignoring orientation

It seems that it's not possible to set orientation to a workspace without any windows o.O
