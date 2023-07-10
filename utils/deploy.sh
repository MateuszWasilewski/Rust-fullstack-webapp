# !bin/bash

cd ~/app
git pull

sudo docker-compose pull
sudo docker-compose up -d --remove-orphans 
sudo docker images prune
