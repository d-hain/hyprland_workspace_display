use hyprland_workspace_display::workspace::Workspace;

pub fn get_workspace_symbols(
    workspaces: &Vec<Workspace>,
    workspace_amount: u8,
    active_workspace_symbol: char,
    full_workspace_symbol: char,
    empty_workspace_symbol: Option<char>,
    active_workspace: &Workspace,
) -> Vec<(char, u8)> {
    let mut symbols = Vec::new();

    for idx in 1..=workspace_amount {
        if let Some(workspace) = get_workspace_by_id(idx, &workspaces) {
            if workspace.id == active_workspace.id {
                symbols.push((active_workspace_symbol, idx));
            } else {
                symbols.push((full_workspace_symbol, idx));
            }
        } else if let Some(symbol) = empty_workspace_symbol {
                symbols.push((symbol, idx));
            }
        }
    }

    return symbols;
}

fn get_workspace_by_id(id: u8, workspaces: &Vec<Workspace>) -> Option<Workspace> {
    for workspace in workspaces {
        if workspace.id == id {
            return Some(workspace.clone());
        }
    }

    return None;
}
