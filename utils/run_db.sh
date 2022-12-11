# !bin/bash

sudo docker run --name postgres-test -p 5432:5432 -e POSTGRES_PASSWORD=admin -d postgres