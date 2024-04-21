#!/bin/bash
zellij plugin -c "session_name=$1,dir=$2" -- "file:$HOME/.cargo/bin/zellij-session-switcher.wasm"
exit 0
