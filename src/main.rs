use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::process::Command;

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
    empty_workspace_symbol: char,

    /// Symbol for a workspace that contains one or more windows.
    #[clap(long)]
    full_workspace_symbol: char,

    /// Symbol for a workspace that is currently being displayed.
    #[clap(long)]
    active_workspace_symbol: char,
}

fn main() {
    let args = Args::parse();

    let workspaces = get_workspaces().expect("got no workspaces");
    let active_workspace = get_active_workspace().expect("got no active workspace");
    let mut print_buffer = String::new();

    for idx in 1..=args.workspace_amount {
        if let Some(workspace) = get_workspace_by_id(idx, &workspaces) {
            if workspace.id == active_workspace.id {
                print_buffer.push(args.active_workspace_symbol);
            } else if workspace.id == idx {
                print_buffer.push(args.full_workspace_symbol);
            }
        } else {
            print_buffer.push(args.empty_workspace_symbol);
        }

        print_buffer.push(' ');
    }

    println!("{}", print_buffer);
}

fn get_workspaces() -> anyhow::Result<Vec<Workspace>> {
    let workspaces = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .context("hyprctl failed to run");

    Ok(serde_json::from_slice(&workspaces?.stdout)?)
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
