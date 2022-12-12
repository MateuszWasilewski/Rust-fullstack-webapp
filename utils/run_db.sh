# !bin/bash

NAME=db-postgres

#-v db-data:/var/lib/postgresql/data \

sudo docker container rm -f $NAME

sudo docker run --name $NAME -p 5432:5432 \
    -e POSTGRES_PASSWORD=admin \
    -e POSTGRES_DB=pgdb \
    -v db-data:/var/lib/postgresql/data \
    -d postgres