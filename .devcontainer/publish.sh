#!/bin/bash

if [[ -z $1 ]]; then
    echo "Please enter a version number for the devcontainer"
    echo "./publish 1"
    exit 1
fi

REMOTE_URL=$(git config --get remote.origin.url)
REMOTE_URL_WITHOUT_SUFFIX=${REMOTE_URL%.git}
REMOTE_URL_WITHOUT_PREFIX=${REMOTE_URL_WITHOUT_SUFFIX#git@github.com:}
REMOTE_REPOSITORY=$(basename ${REMOTE_URL_WITHOUT_SUFFIX,,})
REMOTE_NAMESPACE=$(dirname ${REMOTE_URL_WITHOUT_PREFIX,,})

IMAGE_TAG="ghcr.io/${REMOTE_NAMESPACE}/${REMOTE_REPOSITORY}/rust-dev:${1}"

exists=$(docker manifest inspect "${IMAGE_TAG}" > /dev/null ; echo $?)
if [[ exists -eq 0 ]]; then
    echo -n "Image ${IMAGE_TAG} already exists, proceed to overwrite? [y/N]: "
    read -r ans
    ans=${ans:-n}
    ans=${ans,,}

    if [[ $ans != "y" ]]; then
        echo "Canceling operation"
        exit 0
    else
        echo "overwritting ${IMAGE_TAG}"
    fi
fi

docker build . -t "${IMAGE_TAG}"

echo -n "Ready to publish ${IMAGE_TAG}? [y/N]: "
read -r ans
ans=${ans:-n}
ans=${ans,,}

if [[ $ans != "y" ]]; then
    echo "Canceling operation"
    exit 0
else
    docker push "${IMAGE_TAG}"
fi
