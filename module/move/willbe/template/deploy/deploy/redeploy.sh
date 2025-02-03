#!/bin/sh

docker rm -f ${DOCKER_IMAGE_NAME}
docker rmi ${DOCKER_IMAGE}
docker pull ${DOCKER_IMAGE}
docker run -d --restart unless-stopped -p 80:80 --name=${DOCKER_IMAGE_NAME} ${DOCKER_IMAGE}
