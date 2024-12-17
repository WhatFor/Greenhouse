#!/bin/bash
docker rm -f greenhouse 2>/dev/null || true

docker build -t greenhouse:local . \
    && docker run -d \
        --volume //./pipe/docker_engine:/var/run/docker.sock \
        --name greenhouse \
        greenhouse:local