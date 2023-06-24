# Hyprland Workspace Display

Multiple monitors and special workspaces are not and WILL NOT be supported.
Except someone wants to make a pull request to add that functionality.

Unicode icons are supported.

# TODO
Config file in `${XDG_CONFIG_HOME}/hypr/hyprspacedp/config.toml`.

```shell
hyprspacedp --workspace-amount 10 \
            --empty-workspace-symbol o \
            --full-workspace-symbol 'O' \
            --active-workspace-symbol ""
```

To return eww widgets instead of only letters/icons use the `--eww-widgets` option.
