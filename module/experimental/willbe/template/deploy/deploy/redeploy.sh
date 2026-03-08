#!/bin/sh

docker rmi ${DOCKER_IMAGE}_1
docker rmi ${DOCKER_IMAGE}_2

docker pull ${DOCKER_IMAGE}_1
docker pull ${DOCKER_IMAGE}_2

echo -n "
services:
  one:
    image: ${DOCKER_IMAGE}_1
    ports:
      - "80:80"
  two:
    image: ${DOCKER_IMAGE}_2
    ports:
      - "8080:8080"
" > docker-compose.yml

docker compose up -d
