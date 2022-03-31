#!/bin/sh
export DOCKER_BUILDKIT=1 
podman build --format=docker -t ml -f ./Containerfile ./scripts/