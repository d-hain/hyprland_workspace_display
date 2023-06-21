# Hyprland Workspace Display

Multiple monitors and special workspaces are not and WILL NOT be supported.
Except someone wants to make a pull request to add that functionality.

Config file in `${XDG_CONFIG_HOME}/hypr/hyprspacedp/config.toml`.

Any alpha/opacity color value will be ignored.

```shell
hyprspacedp --workspace-amount 10 \
            --empty-workspace-symbol o \
            --empty-workspace-color "#69696900" \
            --full-workspace-symbol "O" \
            --full-workspace-color "#424242" \
            --active-workspace-symbol "x" \
            --active-workspace-color "#420690"
```
