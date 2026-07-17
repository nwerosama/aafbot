#!/bin/bash

REGISTRY=ghcr.io/nwerosama/aafbot
TAG_NAME=$(git rev-parse --abbrev-ref HEAD)
COMMIT=$(git rev-parse --short HEAD)
echo "Building on $TAG_NAME branch with commit hash $COMMIT"

cargo build --locked -rF production && \
podman build -t $REGISTRY:$TAG_NAME . && podman push $REGISTRY:$TAG_NAME
