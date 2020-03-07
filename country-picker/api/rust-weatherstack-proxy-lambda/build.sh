#!/bin/bash
if [ -z "$1" ]
  then
    echo "Usage: build.sh <api key>"
    exit 1
fi
docker run --rm \
    --env API_KEY=$1 \
    -v ${PWD}:/code \
    -v ${HOME}/.cargo/registry:/root/.cargo/registry \
    -v ${HOME}/.cargo/git:/root/.cargo/git \
    softprops/lambda-rust

