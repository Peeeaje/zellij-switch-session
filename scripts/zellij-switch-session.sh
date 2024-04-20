#!/bin/bash
zellij action start-or-reload-plugin "file:$HOME/.cargo/bin/zellij-session-switcher.wasm" -c "session_name=$1,dir=$2"
