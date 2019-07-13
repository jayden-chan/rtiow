#!/bin/bash

case $1 in
    setup)
        sudo apt update
        sudo apt install gcc htop make
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
        sudo curl -sL https://deb.nodesource.com/setup_12.x | bash -
        sudo apt-get install -y nodejs
        sudo apt-get install ffmpeg
        mkdir out
        ;;
    connect)
        ssh -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP
        ;;
    download)
        scp -i ~/.ssh/gcp_ssh -r jayden@$GCP_VM_IP:/home/jayden/raytracer/$2 ./$2
        ;;
    download-img)
        scp -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP:/home/jayden/raytracer/out/image.ppm ./out/image.ppm
        feh --auto-zoom --force-aliasing out/image.ppm
        ;;
    download-ani)
        scp -i ~/.ssh/gcp_ssh jayden@$GCP_VM_IP:/home/jayden/raytracer/out/output.avi ./out/output.avi
        xdg-open out/output.avi
        ;;
    convert)
        convert out/image.ppm img/$2.png
        ;;
    gen)
        rm -f scenes/animation/*
        rm -f out/*
        node scripts/gen_animation_frames.js

        for f in $(ls scenes/animation/); do
            ./target/release/raytracer "./scenes/animation/$f" "out/$f.ppm"
        done

        ffmpeg -pattern_type glob -framerate 25 -i "out/*.ppm" "out/output.avi"
        ;;
    *)
        echo "unknown command"
esac
