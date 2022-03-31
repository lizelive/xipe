# syntax=docker.io/docker/dockerfile:1
#FROM nvcr.io/nvidia/cuda:11.5.1-cudnn8-runtime-ubuntu20.04
FROM ubuntu:jammy as base


# RUN rm -f /etc/apt/apt.conf.d/docker-clean
# RUN --mount=type=cache,target=/var/cache/apt apt-get update

# install cuda
# RUN --mount=type=cache,target=/var/cache/ export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends nvidia-cudnn


ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONPYCACHEPREFIX="/var/cache/pycache/" \
    DEBIAN_FRONTEND=noninteractive
# SHELL ["/bin/bash", "-c"]

# install nvidia-cudnn python pip
RUN apt-get -q update && apt-get -yq install --no-install-recommends \
    # install ca-certificates 
    ca-certificates \
    apt-transport-https \
    # build essential 
    build-essential \
    # s6 and/or supervisor
    # s6 \
    # install cudnn
    nvidia-cudnn \
    # ssh
    openssh-client \
    openssh-server \
    # install common utils
    ubuntu-standard \
    ubuntu-server \
    ubuntu-minimal \
    # more utils
    locate \
    jq \
    gnupg2 \
    net-tools \
    neofetch \
    unzip zip \
    nano \
    dialog \
    ncdu \
    # graphviz
    graphviz \
    # add more shells
    zsh fizsh \
    fish \
    # install aptitude
    # aptitude debtree
    # install podman
    podman-docker \
    # install python
    python3 \
    # install pip
    python3-pip \
    # update pip
    # && python3 -m pip install --ignore-installed pip \
    && find /usr/lib/python*/ -type d -name "__pycache__" -exec rm -r {} + \
    # && apt-mark auto python3-pip
    # && apt-get -y autoremove --purge \
    # clean up .pyc
    # && find /usr/lib/python*/dist-packages/ -type d -empty -delete \
    && rm -rf $PYTHONPYCACHEPREFIX

FROM base
# ADD ./scripts/ /opt/
COPY ./ /usr/local/bin/

# install common python packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    python3-venv \
    python3-pip \
    python3-torch \
    python3-sklearn \
    python3-sklearn-pandas \
    python3-numpy \
    python3-scipy \
    python3-pandas \
    python3-matplotlib \
    python3-matplotlib-venn \
    python3-matplotlib-inline \
    # python3-keras \
    python3-skorch \
    python3-scikit-fmm \
    python3-scrapy \
    python3-soupsieve \
    python3-autopep8 \
    python3-ipykernel \
    black \
    yapf3 \
    bandit \
    flake8 \
    mypy \
    pycodestyle \
    pydocstyle \
    pylint

# install more python stuff
RUN python3 -m pip install \
    jupyterlab notebook \
    sklearn sklearn-pandas \
    numpy \
    scipy \
    pandas \
    matplotlib \
    matplotlib-venn \
    matplotlib-inline \ 
    keras \
    skorch \
    scikit-learn \
    scrapy \
    soupsieve \
    autopep8 \
    ipykernel \
    black \
    yapf \
    bandit \
    flake8 \
    mypy \
    pycodestyle \
    pydocstyle \
    pylint \
    # no ray for cp310
    # ray \
    # onnx onnxruntime-gpu \
    azureml-core

# setup non-root user
ENV USERNAME=azureuser \
    LANG=en_US.UTF-8 \
    LANGUAGE="" \
    LC_ALL=""

RUN useradd --create-home -ms /bin/bash $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
