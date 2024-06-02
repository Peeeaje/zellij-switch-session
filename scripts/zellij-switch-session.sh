#!/bin/bash
zellij plugin -c "session_name=$1,dir=$2" -- "https://github.com/Peeeaje/zellij-switch-session/releases/download/Latest/zellij-session-switcher.wasm"
exit 0
