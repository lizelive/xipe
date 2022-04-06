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
#TODO python3-opencv
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # essentals
    python3-dev \
    python3-venv \
    python3-pip \
    # ml
    python3-torch \
    python3-opencv \
    python3-sklearn \
    python3-sklearn-pandas \
    python3-numpy \
    python3-scipy \
    python3-pandas \
    python3-matplotlib \
    python3-matplotlib-venn \
    python3-matplotlib-inline \
    # python3-keras \
    # code quality
    python3-skorch \
    python3-scikit-fmm \
    python3-scrapy \
    python3-soupsieve \
    python3-autopep8 \
    python3-ipykernel \
    python3-pytest \
    black \
    yapf3 \
    bandit \
    flake8 \
    mypy \
    pycodestyle \
    pydocstyle \
    pylint \
    # jupyter
    jupyter-notebook 

# azureml deps
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    python3-adal \
    python3-argcomplete \
    python3-bcrypt \
    python3-certifi \
    python3-cffi  \
    python3-charset-normalizer \
    python3-contextlib2 \
    python3-cryptography  \
    python3-docker \
    python3-humanfriendly \
    python3-idna \
    python3-isodate  \
    python3-jeepney \
    python3-jmespath \
    python3-jsonpickle  \
    python3-knack \
    python3-msal \
    # msal extensions is too new
    # python3-msal-extensions \
    python3-msrest  \
    python3-msrestazure \
    python3-ndg-httpsclient \
    python3-oauthlib \
    python3-packaging \
    python3-paramiko \
    python3-pathspec \
    python3-pkginfo \
    python3-portalocker \
    python3-pyasn1 \
    python3-pycparser \
    python3-pygments \
    python3-openssl \
    python3-pyparsing \
    python3-dateutil \
    python3-yaml \
    python3-requests \
    python3-requests-oauthlib \
    python3-secretstorage \
    python3-setuptools  \
    python3-six \
    python3-tabulate \
    python3-urllib3 \
    python3-wheel \
    python3-websocket \
    python3-jwt \
    python3-nacl \
    python3-socks
    # these don't exist becuse using python10
    # python3-backports.tempfile \
    # python3-backports.weakref


# not sure
# python3-pytzdata 

# python3-jwt

# manual
# backports.weakref
 
# install azureml
RUN python3 -m pip install azureml-core

# install torch
# RUN python3 -m pip install torch torchvision torchaudio --extra-index-url https://download.pytorch.org/whl/cu113

# install tensorflow
# RUN python3 -m pip install tensorflow


ENV NVIDIA_VISIBLE_DEVICES all
#  \
    # NVIDIA_REQUIRE_CUDA=cuda>=11.3 \
    # NVIDIA_DRIVER_CAPABILITIES=all


# setup non-root user
ENV USERNAME=azureuser \
    LANG=en_US.UTF-8 \
    LANGUAGE="" \
    LC_ALL=""

RUN useradd --create-home -ms /bin/bash $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

EXPOSE 8888
USER azureuser
WORKDIR /home/azureuser
CMD jupyter notebook --ip='*' --NotebookApp.token='' --NotebookApp.password='' --no-browser