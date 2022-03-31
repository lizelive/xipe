podman run --rm --gpus=all --privileged quay.io/podman/stable podman run --rm --gpus=all nvcr.io/nvidia/cuda:11.5.1-cudnn8-runtime-ubuntu20.04 nvidia-smi
podman run --rm docker.io/nvidia/samples:nbody

podman run --rm --security-opt=label=disable \
     --hooks-dir=/usr/share/containers/oci/hooks.d/ \
     --cap-add SYS_ADMIN nvidia/samples:dcgmproftester-2.0.10-cuda11.0-ubuntu18.04 \
     --no-dcgm-validation -t 1004 -d 30