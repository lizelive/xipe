#!/bin/sh
docker build -t ml --output=plain ./scripts -f ./Dockerfile
alias docker-dive=