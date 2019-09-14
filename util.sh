#!/bin/bash

case $1 in
    provision)
        if [ "$GCP_PROJECT" = "" ] || [ "$GCP_SVC_ACC" ]; then
            echo "GCP env vars not set"
        fi

        gcloud beta compute \
            --project=$GCP_PROJECT \
            instances create rendering-instance \
            --zone=us-west1-b \
            --machine-type=custom-96-88576 \
            --subnet=default \
            --network-tier=PREMIUM \
            --maintenance-policy=MIGRATE \
            --service-account=$GCP_SVC_ACC \
            --scopes=https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/trace.append \
            --image=ubuntu-1804-bionic-v20190911 \
            --image-project=ubuntu-os-cloud \
            --boot-disk-size=10GB \
            --boot-disk-type=pd-standard \
            --boot-disk-device-name=rendering-instance \
            --reservation-affinity=any
        ;;
    setup)
        sudo apt update
        sudo apt install gcc htop make
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        sudo curl -sL https://deb.nodesource.com/setup_12.x | bash -
        sudo apt-get install -y nodejs
        sudo apt-get install ffmpeg
        mkdir out
        echo "Finished setup process. Don't for get to run source \$HOME/.cargo/env"
        ;;
    iter)
        make build \
        && ./target/release/raytracer $2 out/image.ppm \
        && feh --auto-zoom --force-aliasing out/image.ppm
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
    stom)
        echo "$2 / 60; $2 % 60" | bc
        ;;
    gen)
        rm -f scenes/animation/*
        rm -f out/*
        node scripts/animation_1.js

        for f in $(ls scenes/animation/); do
            ./target/release/raytracer "./scenes/animation/$f" "out/$f.ppm"
        done

        ffmpeg -pattern_type glob -framerate 25 -i "out/*.ppm" -c:v libx264 -crf 25 -b:v 40M -pix_fmt yuv420p "out/animation_1.mp4"
        ;;
    *)
        echo "unknown command"
esac
