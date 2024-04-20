#!/usr/bin/env bash

# Change this so that it shows desired directories
selected_path=$(fdfind . ~/work ~/private ~/dotfiles ~/Documents ~ --min-depth 1 --max-depth 1 --type d | fzf)

# If no directory was selected, exit the script
if [[ -z $selected_path ]]; then
    exit 0
fi

# Get the name of the selected directory, replacing "." with "_"
session_name=$(basename $(dirname "$selected_path"))-$(basename "$selected_path" | tr . _)

if [[ -z $ZELLIJ ]]; then
	cd $selected_path
	zellij attach $session_name -c
	exit 0
else
    # Check if we are already in the session
    current_session_name_line=$(zellij ls | grep current | sed 's/\x1b\[[0-9;]*m//g')
    current_session_name=$(echo $current_session_name_line | cut -d " " -f 1)
    if [[ $current_session_name == $session_name ]]; then
        exit 0
    fi

    zellij-switch-session $session_name $selected_path
    exit 0
fi
