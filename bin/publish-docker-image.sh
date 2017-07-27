#! /bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR/..

$(cd backend && cargo clean)

# Go to project dir.
echo `pwd`
docker build -f docker/Dockerfile -t theduke/translator:latest .
docker push theduke/translator:latest
