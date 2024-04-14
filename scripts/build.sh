#!/bin/bash

args_for_binary=()

clear_terminal=false

for arg in "$@"
do
  if [[ $arg == "-v" ]]; then
    clear_terminal=true
  else
    args_for_binary+=("$arg")
  fi
done


    cargo build
if $clear_terminal; then
    clear
fi

./target/debug/smd "${args_for_binary[@]}"
