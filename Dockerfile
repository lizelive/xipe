#FROM nvidia/cuda:11.4.1-cudnn8-runtime-ubuntu20.04
FROM ubuntu:jammy

# install nvida stuff 4.28GiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends nvidia-cudnn

# 80mib
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends supervisor s6 systemd

# install utilities 150MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        apt-utils \
        build-essential \
        openssh-client \
        openssh-server \
        gnupg2 \
        dirmngr \
        iproute2 \
        procps \
        lsof \
        htop \
        net-tools \
        psmisc \
        curl \
        neofetch \
        wget \
        rsync \
        ca-certificates \
        unzip \
        zip \
        nano \
        vim-tiny \
        less \
        jq \
        lsb-release \
        apt-transport-https \
        dialog \
        locales \
        sudo \
        ncdu \
        man-db \
        strace \
        manpages \
        manpages-dev \
        init-system-helpers \
        git \
        manpages-posix \ 
        manpages-posix-dev \
        fish \
        fizsh \
        vim

# more utils 70MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends ubuntu-standard

# dashboards 38MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends systemd-journal-remote freeboard cockpit nginx filetea

# install common python packages 1.27GiB
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

# intall node  71MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        nodejs \
        npm

# install rust langauge 400MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends rust-all

# install common r stuff missing mlr3, xgboost 207mb
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    r-base \
    r-cran-ggplot2 \
    r-cran-data.table \
    r-cran-dplyr \
    r-cran-tidyr \
    r-cran-shiny \
    r-cran-plotly \
    r-cran-knitr \
    r-cran-caret

# install jupyter server 40mib
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    jupyter-notebook 

# graphics libaries 90MiB
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends x11-apps libasound2-dev libxrandr2 libxi-dev libx11-xcb1 freeglut3-dev \
    libxau6 libxdmcp6 libxcb1 libxext6 libx11-6 \
    libglvnd0 libgl1 libglx0 libegl1 libgles2 \
    libglvnd-dev libgl1-mesa-dev libegl1-mesa-dev libgles2-mesa-dev \
    libglx-dev libgl1-mesa-glx libgl1-mesa-dri mesa-utils \
    vulkan-tools libvulkan1 mesa-vulkan-drivers \
    libx11-xcb-dev libxkbcommon-dev libwayland-dev libxrandr-dev libegl1-mesa-dev \
    pulseaudio dbus-x11
ENV NVIDIA_VISIBLE_DEVICES all


# install guac + xrdp 300mb
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    tomcat9 tomcat9-admin tomcat9-common tomcat9-user \
    xrdp guacd libguac-client-rdp0 libguac-client-ssh0 libguac-client-vnc0

# install firefox 119mb
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends firefox

# install xpra 41mb
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends xpra

# install desktop 314mb
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends  budgie-desktop

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends xubuntu-core

RUN export USERNAME=azureuser \
    && useradd --uid 1000 --create-home -ms /bin/bash $USERNAME --groups xpra \
    && mkdir -p /run/user/1000/xpra && chown -R 1000 /run/user/1000 \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
EXPOSE 10000
# 
# # install docker 120MiB
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends \
#         podman-docker \
#         docker-compose

ENV NVIDIA_REQUIRE_CUDA=cuda>=11.4
ENV CUDA_VERSION=11.4.0

ENV CATALINA_BASE=/var/lib/tomcat9
ENV CATALINA_HOME=/usr/share/tomcat9
ENV CATALINA_TMPDIR=/tmp
ENV JAVA_OPTS=-Djava.awt.headless=tru
ENV GUACAMOLE_HOME=/etc/guacamole
# xpra start-desktop --start=startxfce4 --daemon=no

# pulseaudio --start
# sh /usr/share/xrdp/socksetup
# xrdp
# guacd -b 127.0.0.1 -l 4822
# /bin/sh /usr/libexec/tomcat9/tomcat-start.sh
# find /etc/systemd/ -name tomcat*

ADD https://dlcdn.apache.org/guacamole/1.4.0/binary/guacamole-1.4.0.war $CATALINA_BASE/webapps/guacamole.war
# ADD https://dlcdn.apache.org/guacamole/1.4.0/binary/guacamole-auth-header-1.4.0.tar.gz $GUACAMOLE_HOME/extensions/
RUN mkdir -p $GUACAMOLE_HOME/extensions/ && wget -c https://dlcdn.apache.org/guacamole/1.4.0/binary/guacamole-auth-header-1.4.0.tar.gz -O - | tar -xz --strip-components 1 -C $GUACAMOLE_HOME/extensions/

RUN pip install jupyterlab
# uses the REMOTE_USER header
USER azureuser
WORKDIR /home/azureuser
CMD jupyter lab --ip='*' --NotebookApp.token='' --NotebookApp.password='' --no-browser