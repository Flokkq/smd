#!/bin/bash

cargo build --release

npm i -g github-markdown-css

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

sudo mv ../smd /usr/local/bin/
sudo mv target/release/smd /usr/local/bin/

if [ $? -eq 0 ]; then
    echo "Installation successful. You can run the program using 'smd' command."
    rm -rf ../smd
else
    echo "Installation failed."
fi