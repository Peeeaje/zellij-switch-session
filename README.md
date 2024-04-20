## About

This is a [Zellij][zellij] plugin for switching session.

[zellij]: https://github.com/zellij-org/zellij

## Installation
first, you need to add the `wasm32-wasi` target to your rust toolchain by running the following command:
```bash
rustup target add wasm32-wasi
```

then, install the plugin by running the following command:
```bash
cargo install --target wasm32-wasi --path .
```

## Usage
if you want to run the plugin from cli, you can use the following command:
```bash
chmod +x scripts/zellij-switch-session.sh
sudo cp scripts/zellij-switch-session.sh /usr/local/bin/zellij-switch-session
```
and then you can use the alias `zellij-switch-session` to switch session.
```bash
zellij-switch-session <session-name> <dir>
```

## Sessionizer
also if you are interested in [ThePrimeagen] 's [tmux-sessionizer] like functionality, you can use `scripts/zellij-sessionizer.sh` for it.

[ThePrimeagen]: https://github.com/ThePrimeagen/ThePrimeagen
[tmux-sessionizer]: https://github.com/ThePrimeagen/.dotfiles/blob/master/bin/.local/scripts/tmux-sessionizer

if you want to run sessionizer in floating page inside zellij, write below in your config.kdl
```kdl
shared_except "locked" {
    ...
    bind "Ctrl f" {
        Run "zellij-sessionizer" {
            floating true
            close_on_exit true
        }
    }
}
```
