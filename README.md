## About

This is a [Zellij][zellij] plugin for switching session.

[zellij]: https://github.com/zellij-org/zellij

## Installation
Clone and run the following command to install the plugin.
```bash
chmod +x scripts/zellij-switch-session.sh
sudo cp scripts/zellij-switch-session.sh /usr/local/bin/zellij-switch-session
```
and then you can use the alias `zellij-switch-session` to switch session.
```bash
zellij-switch-session <session-name> <dir>
```

Also you can use pipe too.
```bash
zellij pipe -p https://github.com/Peeeaje/zellij-switch-session/releases/download/Latest/zellij-session-switcher.wasm -- <session_name>::<dir>
```

## Sessionizer
also if you are interested in [ThePrimeagen] 's [tmux-sessionizer] like functionality, you can use `scripts/zellij-sessionizer.sh` for it.

[ThePrimeagen]: https://github.com/ThePrimeagen/ThePrimeagen
[tmux-sessionizer]: https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer

if you want to run sessionizer as pane in zellij, add below in your config.kdl
```kdl
shared_except "locked" {
    ...
    bind "Ctrl f" {
        Run "zellij-sessionizer" {
            // if you want to open in floating pane, set `floating true` instead.
            in_place true
            close_on_exit true
        }
    }
}
```
