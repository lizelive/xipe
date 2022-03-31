#!/bin/sh

if ! which curl wget apt-key > /dev/null
then
apt-get update -y && apt-get install curl wget gnupg2 -y
fi

# source podman
VERSION_ID=$(. /etc/os-release;echo $VERSION_ID) \
    && sh -c "echo 'deb http://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/stable/xUbuntu_${VERSION_ID}/ /' > /etc/apt/sources.list.d/devel:kubic:libcontainers:stable.list" \
    && wget -nv https://download.opensuse.org/repositories/devel:kubic:libcontainers:stable/xUbuntu_${VERSION_ID}/Release.key -O- | apt-key add -

# source nvidia-container-toolkit https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html
distribution=$(. /etc/os-release;echo $ID$VERSION_ID) \
    && curl -s -L https://nvidia.github.io/libnvidia-container/gpgkey | sudo apt-key add - \
    && curl -s -L https://nvidia.github.io/libnvidia-container/$distribution/libnvidia-container.list | sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

apt-get update -qq -y
apt-get -qq --yes install podman nvidia-container-toolkit
set -e
podman --version

sudo sed -i 's/^#no-cgroups = false/no-cgroups = true/;' /etc/nvidia-container-runtime/config.toml

mkdir -p /usr/share/containers/oci/hooks.d/
echo '{
    "version": "1.0.0",
    "hook": {
        "path": "/usr/bin/nvidia-container-toolkit",
        "args": ["nvidia-container-toolkit", "prestart"],
        "env": [
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
        ]
    },
    "when": {
        "always": true,
        "commands": [".*"]
    },
    "stages": ["prestart"]
}' > /usr/share/containers/oci/hooks.d/oci-nvidia-hook.json