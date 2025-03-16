#!/bin/bash
set -euo pipefail

IMAGE_NAME="nyx"
TAG="latest"

echo "Building Docker image: $IMAGE_NAME:$TAG"
docker build -t "$IMAGE_NAME:$TAG" .

echo "Build complete!"
echo "To run the container: docker run -p 8080:80 $IMAGE_NAME:$TAG"
