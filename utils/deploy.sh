# !bin/bash

IMAGE=image.tar
VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key
APP_NAME=fullstack-webapp


#sudo docker save $IMAGE | bzip2 | ssh -i $SSH_ID ubuntu@$VM_IP docker load

sudo DOCKER_BUILDKIT=1 docker save -o /home/mateusz/Documents/Code/Rust/fullstack-webapp/$IMAGE $APP_NAME
sudo chown mateusz image.tar
scp -i $SSH_ID $IMAGE ubuntu@$VM_IP:~/images/
rm $IMAGE

ssh -i $SSH_ID ubuntu@$VM_IP " \
    sudo docker image rm -f $APP_NAME:latest; \
    sudo docker load -i images/$IMAGE; \
    sudo docker run -d -p 80:8000 fullstack-webapp"
# on VM
#sudo docker load -i images/image.tar
#sudo docker run -p 80:8000 fullstack-webapp
#logout