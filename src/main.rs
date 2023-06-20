use clap::Parser;
use hex_color::HexColor;
use serde::{Deserialize, Serialize};
use std::{process::Command};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Workspace {
    id: u8,
    name: String,
    monitor: String,
    windows: u32,
    #[serde(rename = "hasfullscreen")]
    has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    last_window: String,
    #[serde(rename = "lastwindowtitle")]
    last_window_title: String,
}

/// Hyprland Workspace Display
/// Outputs workspace symbols for hyprland that can be colored,
/// depending on the state of the them.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Amount of workspace symbols to be displayed.
    #[clap(short = 'n', long)]
    workspace_amount: u8,

    /// Symbol for a workspace that does not contain any windows.
    #[clap(long)]
    empty_workspace_symbol: String,

    /// Hex color for a workspace that does not contain any windows.
    #[clap(long, default_value_t = HexColor::WHITE)]
    empty_workspace_color: HexColor,

    /// Symbol for a workspace that contains one or more windows.
    #[clap(long)]
    full_workspace_symbol: String,

    /// Hex color for a workspace that contains one or more windows.
    #[clap(long, default_value_t = HexColor::WHITE)]
    full_workspace_color: HexColor,

    /// Symbol for a workspace that is currently being displayed.
    #[clap(long)]
    active_workspace_symbol: String,

    /// Hex color for a workspace that is currently being displayed.
    #[clap(long, default_value_t = HexColor::WHITE)]
    active_workspace_color: HexColor,
}

fn main() {
    let args = Args::parse();

    let workspaces = get_workspaces().expect("got no workspaces");
    let active_workspace = get_active_workspace().expect("got no active workspace");
    let mut print_buffer = String::new();
    for idx in 1..=args.workspace_amount {
        dbg!(idx);
        if let Some(workspace) = get_workspace_by_id(idx, &workspaces) {
            if workspace.id == active_workspace.id {
                print_buffer.push_str(&args.active_workspace_symbol);
            } else if workspace.id == idx {
                print_buffer.push_str(&args.full_workspace_symbol);
            }
        } else {

                print_buffer.push_str(&args.empty_workspace_symbol);
            }
    }

    println!("{}", print_buffer);
}

fn get_workspaces() -> Option<Vec<Workspace>> {
    let workspaces = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .expect("hyprctl failed to run");

    serde_json::from_slice(&workspaces.stdout).ok()
}

fn get_workspace_by_id(id: u8, workspaces: &Vec<Workspace>) -> Option<Workspace> {
    for workspace in workspaces {
        if workspace.id == id {
            return Some(workspace.clone());
        }
    }

    return None;
}

fn get_active_workspace() -> Option<Workspace> {
    let workspace = Command::new("hyprctl")
        .arg("activeworkspace")
        .arg("-j")
        .output()
        .expect("hyprctl failed to run");

    serde_json::from_slice(&workspace.stdout).ok()
}
