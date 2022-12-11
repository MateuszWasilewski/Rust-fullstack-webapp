# !bin/bash
# --platform linux/aarch64
sudo docker image rm fullstack-webapp:latest
sudo DOCKER_BUILDKIT=1 docker build -t fullstack-webapp .