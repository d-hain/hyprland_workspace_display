use anyhow::Context;
use clap::Parser;
use hyprland_workspace_display::workspace::Workspace;
use std::process::Command;

mod symbols;

/// Outputs symbols or eww widgets for each workspace, depending on them
/// containing a window or being the currently active one.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Amount of workspace symbols to be displayed.
    #[clap(short = 'n', long)]
    workspace_amount: u8,

    /// Symbol for a workspace that does not contain any windows.
    #[clap(long)]
    empty_symbol: char,

    /// Symbol for a workspace that contains one or more windows.
    #[clap(long)]
    full_symbol: char,

    /// Symbol for a workspace that is currently active.
    #[clap(long)]
    active_symbol: char,

    /// Return eww widgets (buttons) instead of only symbols for each workspace.
    #[clap(long)]
    eww_widgets: bool,

    /// Runs continuously and prints on change in workspaces.
    #[clap(short = 'p', long)]
    polling: bool,
}

fn main() {
    let args = Args::parse();

    let workspaces = get_workspaces().expect("got no workspaces");
    let mut active_workspace = get_active_workspace().expect("got no active workspace");
    let mut symbols = symbols::get_workspace_symbols(
        &workspaces,
        args.workspace_amount,
        args.active_symbol,
        args.full_symbol,
        args.empty_symbol,
        &active_workspace,
    );

    if args.polling {
        print_symbols(&symbols, args.eww_widgets);

        loop {
            let workspaces_polling = get_workspaces().expect("got no workspaces");
            let prev_active_workspace = active_workspace.clone();
            active_workspace = get_active_workspace().expect("got no active workspace");

            if workspaces_polling.len() != workspaces.len()
                || prev_active_workspace.id != active_workspace.id
            {
                symbols = symbols::get_workspace_symbols(
                    &workspaces_polling,
                    args.workspace_amount,
                    args.active_symbol,
                    args.full_symbol,
                    args.empty_symbol,
                    &active_workspace,
                );

                print_symbols(&symbols, args.eww_widgets);
            }
        }
    } else {
        print_symbols(&symbols, args.eww_widgets);
    }
}

/// Prints the symbols or the eww widgets for all workspaces.
///
/// # Arguments
///
/// - `workspace_symbols` - the current symbols for all workspaces.
/// - `eww_widgets` - wheter to print eww widgets or normal symbols.
fn print_symbols(workspace_symbols: &Vec<char>, eww_widgets: bool) {
    if eww_widgets {
        for (mut idx, symbol) in workspace_symbols.iter().enumerate() {
            idx += 1;

            println!(
                "(button \
                    :onclick \"hyprctl dispatch workspace {}\" \
                    :class \"workspace-{}\" \
                    :halign \"center\" \
                    \"{}\")",
                idx, idx, symbol
            );
        }
    } else {
        workspace_symbols
            .iter()
            .for_each(|symbol| print!("{} ", symbol));
    }
    println!();
}

/// # Returns
///
/// all workspaces that currently have a window opened on them.
fn get_workspaces() -> anyhow::Result<Vec<Workspace>> {
    let workspaces = Command::new("hyprctl")
        .arg("workspaces")
        .arg("-j")
        .output()
        .context("hyprctl failed to run");

    Ok(serde_json::from_slice(&workspaces?.stdout)?)
}

/// # Returns
///
/// the currently active workspace.
fn get_active_workspace() -> anyhow::Result<Workspace> {
    let workspace = Command::new("hyprctl")
        .arg("activeworkspace")
        .arg("-j")
        .output()
        .expect("hyprctl failed to run");

    Ok(serde_json::from_slice(&workspace.stdout)?)
}
