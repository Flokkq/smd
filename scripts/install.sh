#!/bin/sh

binary_name="smd"

if [ "$(id -u)" -ne 0 ]; then
    echo "This script needs to be run with sudo permissions."
    echo "It will execute: sudo mv target/release/$binary_name /usr/local/bin/"
    exit 1
fi

if ! command -v cargo &> /dev/null
then
    echo "cargo could not be found"
    exit 1
fi

cargo build --release

if [ $? -eq 0 ]; then
    if [ -e "/usr/local/bin/$binary_name" ]; then
        echo "Error: A binary with the name '$binary_name' already exists at /usr/local/bin/"
        echo "Please remove the existing binary or rename the new one before installation."
        exit 1
    fi
    echo "I will execute: sudo mv target/release/$binary_name /usr/local/bin/"
    sudo mv "target/release/$binary_name" /usr/local/bin/
    echo "Installation Successful"
else
    echo "Build failed"
    exit 1
fi
