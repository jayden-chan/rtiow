#!/bin/bash

case $1 in
    setup)
        sudo apt update
        sudo apt install gcc htop
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
        mkdir out
        ;;
    connect)
        ssh -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP
        ;;
    download)
        scp -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP:/home/jayden/raytracer/out/image.ppm ./out/image.ppm
        ;;
    convert)
        convert out/image.ppm img/$2.png
        ;;
    *)
        echo "unknown command"
esac
