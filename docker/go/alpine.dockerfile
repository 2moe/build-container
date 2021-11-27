# syntax=docker/dockerfile:1

FROM --platform=${TARGETPLATFORM} golang:alpine
ADD go.go /root/README.go

WORKDIR /go
ENV HOME=/root \
    TMOE_PROOT=false \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true
RUN apk update; \
    apk upgrade; \
    apk add sudo tar grep curl wget bash tzdata newt shadow; \
    printf "%s\n" "root:root" | chpasswd; \
    ln -svf /usr/share/zoneinfo/UTC /etc/localtime; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=go_nogui-alpine" \
    > container.txt ; \
    mkdir -p environment; \
    printf "%s\n" \
    'export PATH="/go/bin:/usr/local/go/bin${PATH:+:${PATH}}"' \
    "export GOPATH=/go" \
    > environment/container.env; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "GO_VERSION='$(go version)'" \
    > version.txt; \
    cat version.txt; \
    rm -rf /var/cache/apk/* ~/.cache/* 2>/dev/null

# ENV PATH=/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
#     GOPATH=/go
CMD [ "/bin/bash" ]
