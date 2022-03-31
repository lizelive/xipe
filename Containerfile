# syntax=docker.io/docker/dockerfile:1
#FROM nvcr.io/nvidia/cuda:11.5.1-cudnn8-runtime-ubuntu20.04
FROM ubuntu:jammy


# RUN rm -f /etc/apt/apt.conf.d/docker-clean
# RUN --mount=type=cache,target=/var/cache/apt apt-get update

# install cuda
# RUN --mount=type=cache,target=/var/cache/ export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends nvidia-cudnn


ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONPYCACHEPREFIX="/var/cache/pycache/"
ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-c"]

# install nvidia-cudnn python pip
RUN apt-get update && apt-get -y install --no-install-recommends \
    # install ca-certificates 
    ca-certificates \
    # install cudnn
    # nvidia-cudnn \
    # install podman
    podman-docker \
    # install common utils
    ubuntu-standard \
    ubuntu-server \
    # install python
    python3 \
    # install pip
    python3-pip \
    # update pip
    && python3 -m pip install --ignore-installed pip \
    && find /usr/lib/python*/ -type d -name "__pycache__" -exec rm -rv {} + \
    && apt-get -y purge --auto-remove python3-pip \
    # clean up .pyc
    # && find /usr/lib/python*/dist-packages/ -type d -empty -delete \
    && rm -rf $PYTHONPYCACHEPREFIX

# ADD ./scripts/ /opt/
COPY ./ /usr/local/bin/


# upgrade pip
# RUN --mount=type=cache,target=/root/.cache \
#     python3 -m pip install --upgrade pip setuptools wheel