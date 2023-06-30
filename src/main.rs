use anyhow::Context;
use clap::Parser;
use hyprland_workspace_display::workspace::Workspace;
use std::process::Command;

mod symbols;

/// Outputs symbols or eww widgets for each workspace, depending on them
/// containing a window or being the currently active one.
#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Amount of workspace symbols to be displayed.
    #[arg(short = 'n', long, required = true, value_name = "AMOUNT")]
    workspace_amount: u8,

    /// Symbol for a workspace that does not contain any windows.
    #[arg(long, required = true, value_name = "SYMBOL")]
    empty_symbol: char,

    /// Symbol for a workspace that contains one or more windows.
    #[arg(long, required = true, value_name = "SYMBOL")]
    full_symbol: char,

    /// Symbol for a workspace that is currently active.
    #[arg(long, required = true, value_name = "SYMBOL")]
    active_symbol: char,

    /// Return eww widgets (buttons) instead of only symbols for each workspace.
    #[arg(long)]
    eww_widgets: bool,

    /// CSS class to use for the box.
    #[arg(long, requires = "eww_widgets", value_name = "CLASS")]
    eww_class_box: Option<String>,

    /// Valign to use for the box.
    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "VALIGN",
        value_parser = clap::builder::PossibleValuesParser::new([
            "fill",
            "baseline",
            "center",
            "start",
            "end",
        ])
    )]
    eww_valign: Option<String>,

    /// Halign to use for the box.
    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "HALIGN",
        value_parser = clap::builder::PossibleValuesParser::new([
            "fill",
            "baseline",
            "center",
            "start",
            "end",
        ])
    )]
    eww_halign: Option<String>,

    /// Spacing to use for the box.
    #[arg(long, requires = "eww_widgets", value_name = "SPACING")]
    eww_spacing: Option<u32>,

    /// Orientation to use for the box.
    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "ORIENTATION",
        value_parser = clap::builder::PossibleValuesParser::new([
            "horizontal",
            "h",
            "vertical",
            "v",
        ])
    )]
    eww_orientation: Option<String>,

    /// CSS class to use for the button.
    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "CLASS",
        long_help = "
        It is possible to specify the workspace id by using {NR}.\r
        {NR} will be replaced by the id of the workspace."
    )]
    eww_class_button: Option<String>,

    /// Onclick command to execute for each button.
    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "ONCLICK",
        long_help = "
        It is possible to specify the workspace id by using {NR}.\r
        {NR} will be replaced by the id of the workspace."
    )]
    eww_onclick: Option<String>,

    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "ONMIDDLECLICK",
        long_help = "
        It is possible to specify the workspace id by using {NR}.\r
        {NR} will be replaced by the id of the workspace."
    )]
    eww_onmiddleclick: Option<String>,

    #[arg(
        long,
        requires = "eww_widgets",
        value_name = "ONRIGHTCLICK",
        long_help = "
        It is possible to specify the workspace id by using {NR}.\r
        {NR} will be replaced by the id of the workspace."
    )]
    eww_onrightclick: Option<String>,
}

fn main() {
    let args = Args::parse();

    let workspaces = get_workspaces().expect("got no workspaces");
    let active_workspace = get_active_workspace().expect("got no active workspace");
    let symbols = symbols::get_workspace_symbols(
        &workspaces,
        args.workspace_amount,
        args.active_symbol,
        args.full_symbol,
        args.empty_symbol,
        &active_workspace,
    );

    print_symbols(&symbols, &args);
}

/// Prints the symbols or the eww widgets for all workspaces.
///
/// # Arguments
///
/// - `workspace_symbols` - the current symbols for all workspaces.
/// - `eww_widgets` - wheter to print eww widgets or normal symbols.
fn print_symbols(workspace_symbols: &Vec<char>, args: &Args) {
    if args.eww_widgets {
        println!(
            "{}",
            get_eww_widget(
                workspace_symbols,
                &args.eww_class_box,
                &args.eww_valign,
                &args.eww_halign,
                &args.eww_spacing,
                &args.eww_orientation,
                &args.eww_class_button,
                &args.eww_onclick,
                &args.eww_onmiddleclick,
                &args.eww_onrightclick,
            )
        );
    } else {
        workspace_symbols
            .iter()
            .for_each(|symbol| print!("{} ", symbol));
        println!();
    }
}

fn get_eww_widget(
    workspace_symbols: &Vec<char>,
    eww_class_box: &Option<String>,
    eww_valign: &Option<String>,
    eww_halign: &Option<String>,
    eww_spacing: &Option<u32>,
    eww_orientation: &Option<String>,
    eww_class_button: &Option<String>,
    eww_onclick: &Option<String>,
    eww_onmiddleclick: &Option<String>,
    eww_onrightclick: &Option<String>,
) -> String {
    let mut eww_widget = String::new();

    let push_string_property = |mut eww_widget: String,
                                property: &str,
                                value: &Option<String>,
                                nr: Option<usize>|
     -> String {
        if let Some(mut value) = value.clone() {
            if let Some(nr) = nr {
                value = value.replace("{NR}", &nr.to_string().as_str());
            }
            eww_widget.push_str(&format!(":{} '{}' ", property, value));
        }

        return eww_widget;
    };

    eww_widget.push_str("(box ");

    // Box properties
    eww_widget = push_string_property(eww_widget.clone(), "class", eww_class_box, None);
    eww_widget = push_string_property(eww_widget.clone(), "valign", eww_valign, None);
    eww_widget = push_string_property(eww_widget.clone(), "halign", eww_halign, None);
    if let Some(spacing) = eww_spacing {
        eww_widget.push_str(&format!(":spacing {} ", spacing));
    }
    eww_widget = push_string_property(eww_widget.clone(), "orientation", eww_orientation, None);

    // Buttons
    for (mut idx, symbol) in workspace_symbols.iter().enumerate() {
        idx += 1;

        eww_widget.push_str("(button ");

        eww_widget = push_string_property(eww_widget.clone(), "class", eww_class_button, Some(idx));
        eww_widget = push_string_property(eww_widget.clone(), "onclick", eww_onclick, Some(idx));
        eww_widget = push_string_property(
            eww_widget.clone(),
            "onmiddleclick",
            eww_onmiddleclick,
            Some(idx),
        );
        eww_widget = push_string_property(
            eww_widget.clone(),
            "onrightclick",
            eww_onrightclick,
            Some(idx),
        );

        eww_widget.push_str(&format!("'{}')", symbol));
    }

    eww_widget.push_str(")");

    eww_widget
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
