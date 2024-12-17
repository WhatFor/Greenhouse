#!/bin/bash
docker rm -f greenhouse 2>/dev/null || true

docker build -t greenhouse:local . \
    && docker run -d \
        --volume /var/run/docker.sock:/var/run/docker.sock \
        --rm \
        --name greenhouse \
        greenhouse:local