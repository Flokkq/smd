#!/bin/bash

# Compile the C program along with noc.c from the /lib directory
gcc -I/lib smd.c /lib/noc.c -o smd

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

# Optionally, you can add checks to see if the binary already exists in the target directory

# Move the binary to a directory in PATH (e.g., /usr/local/bin)
# Requires sudo privileges to move to such directories
sudo mv smd /usr/local/bin/

# Verify if the move was successful
if [ $? -eq 0 ]; then
    echo "Installation successful. You can run the program using 'smd' command."
else
    echo "Installation failed."
fi
