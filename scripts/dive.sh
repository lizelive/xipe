#!/bin/sh
docker run --rm -it \
-v /var/run/docker.sock:/var/run/docker.sock \
-e DOCKER_API_VERSION=$(docker version --format '{{.Server.APIVersion}}') \
-v "$(pwd)":"/host$(pwd)" \
-w "/host$(pwd)" \
wagoodman/dive:latest "$@"
