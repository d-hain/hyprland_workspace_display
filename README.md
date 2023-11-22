# Hyprland Workspace Display
# hyprspacedp

[![crates.io](https://img.shields.io/crates/v/hyprland-workspace-display.svg)](https://crates.io/crates/hyprland-workspace-display)

This is a utility for getting symbols for [Hyprland](https://github.com/hyprwm/Hyprland) workspaces. \
Getting [eww](https://github.com/elkowar/eww) widgets (buttons in a box) for the workspaces is also possible and rather customizable. \
Unicode icons are also supported.

# Info

## ⚠️ I am not using Hyprland anymore so there will most likely no more updates ⚠️
But still if any features are missing or there are any bugs please fell free to just open an issue. \
Pull requests are also always welcome.

Multiple monitors and special workspaces are not and WILL NOT be supported. \
Except someone wants to make a pull request to add that functionality.

## My setup (without any css) using the `--eww-widgets` option

![My Setup](/my-setup.png)

```yuck
(defpoll workspaces_yuck 
  :interval "100ms"
  `hyprspacedp \
      -n 10 \
      --full-symbol O \
      --empty-symbol o \
      --active-symbol X \
      --eww-widgets \
      --eww-class-box box-workspaces \
      --eww-halign center \
      --eww-orientation vertical \
      --eww-class-button button-workspace-{NR} \
      --eww-onclick 'hyprctl dispatch workspace {NR}'`)
```

## Examples

`--help` is your best friend.

### Basic symbols
```shell
hyprspacedp --workspace-amount 10 \
            --empty-symbol o \
            --full-symbol 'O' \
            --active-symbol "α"
```

`--workspace-amount` can also be shortened to `-n`.

**Output:**
```
O α O O o o o o o o
```

### Basic Eww widgets

```shell
hyprspacedp -n 10 \
            --empty-symbol o \
            --full-symbol 'O' \
            --active-symbol "α" \
            --eww-widgets
```

**Output:**
```yuck
(box (button 'O')(button 'α')(button 'O')(button 'O')(button 'o')(button 'o')(button 'o')(button 'o')(button 'o')(button 'o'))
```

### Eww widgets with box customizations

```shell
hyprspacedp -n 10 \
            --empty-symbol o \
            --full-symbol 'O' \
            --active-symbol "α" \
            --eww-widgets \
            --eww-class-box workspaces \
            --eww-orientation vertical
```

**Formatted Output:** \
(normally it is all in one line)
```yuck
(box 
  :class 'workspaces' 
  :orientation 'vertical' 
  (button 'O')(button 'α')(button 'O')(button 'O')(button 'o')(button 'o')(button 'o')(button 'o')(button 'o')(button 'o'))
```

### Eww widgets with button customizations

```shell
hyprspacedp -n 10 \
            --empty-symbol o \
            --full-symbol O \
            --active-symbol α \
            --eww-widgets \
            --eww-class-button "workspac -{NR}" \
            --eww-onclick "hyprctl dispatch workspace {NR}"
```

**Formatted Output:** \
(normally it is all in one line)
```yuck
(box 
(button 
  :class 'workspace-1' 
  :onclick 'hyprctl dispatch workspace 1'
  'O')
(button 
  :class 'workspace-2' 
  :onclick 'hyprctl dispatch workspace 2' 
  'α')
(button :class 'workspace-3' :onclick 'hyprctl dispatch workspace 3' 'O')(button :class 'workspace-4' :onclick 'hyprctl dispatch workspace 4' 'O')(button :class 'workspace-5' :onclick 'hyprctl dispatch workspace 5' 'o')(button :class 'workspace-6' :onclick 'hyprctl dispatch workspace 6' 'o')(button :class 'workspace-7' :onclick 'hyprctl dispatch workspace 7' 'o')(button :class 'workspace-8' :onclick 'hyprctl dispatch workspace 8' 'o')(button :class 'workspace-9' :onclick 'hyprctl dispatch workspace 9' 'o')(button :class 'workspace-10' :onclick 'hyprctl dispatch workspace 10' 'o'))
```
