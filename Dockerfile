# syntax=docker.io/docker/dockerfile:1
#FROM nvcr.io/nvidia/cuda:11.5.1-cudnn8-runtime-ubuntu20.04
FROM ubuntu:jammy


# RUN rm -f /etc/apt/apt.conf.d/docker-clean
# RUN --mount=type=cache,target=/var/cache/apt apt-get update

# install cuda
# RUN --mount=type=cache,target=/var/cache/ export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends nvidia-cudnn


ENV PYTHONDONTWRITEBYTECODE=1
# ENV PYTHONPYCACHEPREFIX="/var/cache/pycache/"

SHELL ["/bin/bash", "-c"]

# install nvidia-cudnn python pip
RUN --mount=type=cache,target=/var/cache/ \
    --mount=type=cache,target=/root/.cache \
    export DEBIAN_FRONTEND=noninteractive \
    export PYTHONPYCACHEPREFIX=/var/cache/pycache/ \
    && apt-get update \
    && apt-get -y install --no-install-recommends \
    # install ca-certificates 
    ca-certificates \
    # install cudnn
    nvidia-cudnn \
    # install podman
    podman-docker \
    # install python
    python3 \
    # install pip
    python3-pip \
    && python3 -m pip install --ignore-installed pip \
    && apt-get -y purge --auto-remove python3-pip \
    # clean up .pyc
    && find /usr/lib/python*/ -type f -name "*.py[co]" -delete \
    && find /usr/lib/python*/dist-packages/ -type d -empty -delete

# ADD ./scripts/ /opt/
COPY ./ /usr/local/bin/


# upgrade pip
# RUN --mount=type=cache,target=/root/.cache \
#     python3 -m pip install --upgrade pip setuptools wheel