#!/bin/bash

cargo build --release

npm i -g github-markdown-css

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

<<<<<<< Updated upstream:old/scripts/install.sh
sudo mv target/release/smd /usr/local/bin/

if [ $? -eq 0 ]; then
    echo "Installation successful. You can run the program using 'smd' command."
=======
sudo cp target/release/smd /usr/local/bin/

if [ $? -eq 0 ]; then
    echo "Installation successful. You can run the program using 'smd' command."
    sudo mv ../smd /usr/local/bin/
>>>>>>> Stashed changes:scripts/init.sh
else
    echo "Installation failed."
fi