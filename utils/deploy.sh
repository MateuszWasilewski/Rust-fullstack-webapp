# !bin/bash

IMAGE=image.tar
VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key
APP_NAME=rust-server


#sudo docker save $IMAGE | bzip2 | ssh -i $SSH_ID ubuntu@$VM_IP docker load

sudo DOCKER_BUILDKIT=1 docker save -o /home/mateusz/Documents/Code/Rust/fullstack-webapp/$IMAGE $APP_NAME
sudo chown mateusz image.tar
scp -i $SSH_ID $IMAGE ubuntu@$VM_IP:~/images/
scp -i $SSH_ID docker-compose.yml ubuntu@$VM_IP:~/app/
rm $IMAGE

ssh -i $SSH_ID ubuntu@$VM_IP " \
    sudo docker image rm -f $APP_NAME:latest; \
    sudo docker load -i images/$IMAGE; \
    cd app; \
    sudo docker-compose down; \
    sudo docker-compose up"
# on VM
#sudo docker load -i images/image.tar
#sudo docker run -p 80:8000 fullstack-webapp
#logout