# Hyprland Workspace Display

Multiple monitors and special workspaces are not and WILL NOT be supported.
Except someone wants to make a pull request to add that functionality.

Config file in `${XDG_CONFIG_HOME}/hypr/hyprspacedp/config.toml`.

For a unicode icon use `\u{UNICODE}` like for example the Rust logo: `\u{e7a8}`.

```shell
hyprspacedp --workspace-amount 10 \
            --empty-workspace-symbol o \
            --full-workspace-symbol "O" \
            --active-workspace-symbol "x" \
```
