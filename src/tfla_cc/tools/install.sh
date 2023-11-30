#!/bin/bash

set -e

echo "Starting installation..."

command_exists() {
  command -v "$@" >/dev/null 2>&1
}

main() {
    if ! command_exists git; then 
        echo "You don't have git installed, the installer needs it."
        exit
    fi

    if ! command_exists cargo; then
        echo "You don't have cargo, we can't compile the TFLA CC"
        exit
    fi
    
    git clone https://github.com/LucasOliveiraaa/TFLA.git 

    cd TFLA/src/tfla_cc/

    cargo build --release

    cd target/release/

    sudo mv ./tfla-cc /usr/local/bin/

    cd ../..

    sudo rm -r TFLA

    echo "Installation compleate!"
    echo "Run tfla-cc to test it."
}

main
