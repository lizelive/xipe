#!/bin/sh
DOCKER_API_VERSION=$(docker version --format '{{.Server.APIVersion}}')
PODMAN_API_VERSION=$(podman version --format '{{.Server.APIVersion}}')

# in case the docker prints something to
if [ $? -ne 0 ]
then
   DOCKER_API_VERSION=
fi
podman run --rm -it -v /run/podman/podman.sock:/var/run/docker.sock -e DOCKER_API_VERSION docker.io/wagoodman/dive:latest "$@"
#docker run --rm -it -v /var/run/docker.sock:/var/run/docker.sock -e DOCKER_API_VERSION wagoodman/dive:latest "$@"