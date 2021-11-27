# syntax=docker/dockerfile:1

# DOCKER_BUILDKIT=1 docker build -t u2 -f ../arch/base.dockerfile --platform=linux/amd64 .
# --build-arg ARCH amd64

FROM --platform="$TARGETPLATFORM" alpine:edge 

RUN apk add bash \
    && echo hello world

CMD ["bash"]
