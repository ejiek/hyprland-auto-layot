# Hyprland auto layout

Optimizes your multi-monitor setup by automagically aligning the master layout orientation with respective monitor orientation.
Started from a [Hyprland issue](https://github.com/hyprwm/Hyprland/issues/3174).

## Assumptions

- A multi-monitor setup
- Monitors have different orientation
- Master layout utilization
- ⏳ *Temporary*
  - Central orientation for horizontal
  - Top orientation for vertical

## Roadmap

- [ ] Add proper cli support with [clap](https://github.com/clap-rs/clap)
- [ ] Implement Fire once strategy to handle initial setup
- [ ] Transition from hyprland-rs to reduce size (this tool needs just a couple of dispatchers and a little bit of unix socket parsing)
- [ ] Startup checks & warnings for Fire once
- [ ] Make orientation configurable (currently hardcoded)
- [ ] Documentation (installation, usage, configuration)
- [ ] Figure out Special Workspaces

## How does it work

The recommended way to launch the utility is the `exec-once` directive in `hyprland.conf`.
It parses current Hyprland state for monitors with transform `1`,`3`,`5` or `7` and remembers them.
Then there are two strategies.

### Fire once

On Hyprland startup this app goes through all workspaces and sets appropriate orientation.
Then returns to default workspaces for each monitor.

**Pros:**
- Resource-efficient

**Cons:**
- Not adaptive (e.g., reassigning workspaces to different monitors, accommodating new monitors)
- Initialization takes some time and may look weird (dancing cursor)

**When to use**
Best suited for static workspace-to-monitor bindings

### Daemonized

Listens for Hyprland IPC events.
On `workspace` checks monitor+workspace pair and sets default or vertical orientation.
On `monitoradded` and `monitorremoved' updates a list of vertical monitors.
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

## Extras

Hyprland doesn't report all workspaces through IPC (and hyprctl).
It reports only:

- workspaces with windows
- visible workspaces

Good thing is, it stores orientation even for workspaces it doesn't report.
This is why fire once strategy works. But to find all the workspaces this tool needs to parse `hyprland.conf`.

It seems that it's not possible to set orientation to a workspace without any windows o.O
