# !bin/bash

IMAGE=image.tar
VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key

sudo chown mateusz image.tar

scp -i $SSH_ID $IMAGE ubuntu@$VM_IP:~/images/