# !bin/bash

VM_IP=138.2.182.80
SSH_ID=~/.ssh/oracle_vm.key

RESULT=package.zip 
FILES=".cargo backend common files frontend utils .dockerignore .env .gitignore Cargo.toml Dockerfile"

zip -r ${RESULT} ${FILES}
scp -i $SSH_ID $RESULT ubuntu@$VM_IP:~/images/
rm ${RESULT}  