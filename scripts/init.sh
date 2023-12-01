#!/bin/bash

cargo build --release

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

# Move the binary to a directory in PATH (e.g., /usr/local/bin)
# Requires sudo privileges to move to such directories
sudo mv target/release/smd /usr/local/bin/

# Verify if the move was successful
if [ $? -eq 0 ]; then
    echo "Installation successful. You can run the program using 'smd' command."
else
    echo "Installation failed."
fi
