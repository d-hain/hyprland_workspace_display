use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
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

fn main() {
    let workspaces = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .expect("hyprctl failed to run");

    let workspaces: Vec<Workspace> =
        serde_json::from_slice(&workspaces.stdout).expect("hyprctl had no output");
}
