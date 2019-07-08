#!/bin/bash

case $1 in
    setup)
        sudo apt update
        sudo apt install gcc htop make
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
        sudo curl -sL https://deb.nodesource.com/setup_12.x | bash -
        sudo apt-get install -y nodejs
        mkdir out
        ;;
    connect)
        ssh -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP
        ;;
    download)
        scp -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP:/home/jayden/raytracer/out/image.ppm ./out/image.ppm
        feh --auto-zoom --force-aliasing out/image.ppm
        ;;
    convert)
        convert out/image.ppm img/$2.png
        ;;
    *)
        echo "unknown command"
esac
