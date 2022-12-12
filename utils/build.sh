# !bin/bash
# --platform linux/aarch64
APP_NAME=rust-server

sudo docker image rm $APP_NAME:latest
sudo DOCKER_BUILDKIT=1 docker build -t $APP_NAME .