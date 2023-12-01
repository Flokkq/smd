#!/bin/bash

echo "Building the project..."
cargo build


# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful. Running the binary..."
    target/debug/smd "$@"
else
    echo "Build failed."
fi
