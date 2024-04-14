#!/bin/bash

TARGET_DIR="/Users/clemensweber/Library/Application Support/smd"

if [ -d "$TARGET_DIR" ]; then
    read -p "Are you sure you want to permanently delete \"$TARGET_DIR\"? (y/n): " confirmation
    if [ "$confirmation" = "y" ] || [ "$confirmation" = "Y" ]; then
        rm -rf "$TARGET_DIR"
        echo "Directory removed."
    else
        echo "Operation canceled."
    fi
else
    echo "Directory does not exist or has already been removed."
fi
