#! /bin/bash

TAG=$1

if [ -z $TAG ]; then
    echo "Usage: ./publish-docker-image.sh TAG"
    exit 1;
fi

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR/..

$(cd backend && cargo clean)

# Go to project dir.
echo `pwd`
docker build -f docker/Dockerfile -t theduke/translator:$TAG .
docker push theduke/translator:$TAG
