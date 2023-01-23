# !bin/bash

IMAGE=image.tar
VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key
APP_NAME=rust-server


#sudo docker save $IMAGE | bzip2 | ssh -i $SSH_ID ubuntu@$VM_IP docker load

./utils/zip.sh
#sudo DOCKER_BUILDKIT=1 docker save -o /home/mateusz/Documents/Code/Rust/fullstack-webapp/$IMAGE $APP_NAME
#sudo chown mateusz image.tar
#scp -i $SSH_ID $IMAGE ubuntu@$VM_IP:~/images/
scp -i $SSH_ID docker-compose.yml ubuntu@$VM_IP:~/app/
#scp -i $SSH_ID files/db/* ubuntu@$VM_IP:~/app/files/db
#scp -i $SSH_ID -r photos ubuntu@$VM_IP:~/app/public/

ssh -i $SSH_ID ubuntu@$VM_IP " \
    rm -rf build && \
    mkdir build && \
    mv images/package.zip build/ && \
    cd build && \
    unzip package.zip && \
    ./utils/build.sh && \
    cd ../app && \
    sudo docker-compose down && \
    sudo docker-compose up"

# on VM
#sudo docker load -i images/image.tar
#sudo docker run -p 80:8000 fullstack-webapp
#logout