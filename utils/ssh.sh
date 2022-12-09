# !bin/bash

VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key

ssh -i $SSH_ID ubuntu@$VM_IP