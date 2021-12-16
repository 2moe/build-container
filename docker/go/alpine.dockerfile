# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} golang:alpine
COPY go.go /root/readme.go

WORKDIR /go
ENV LANG="C.UTF-8" \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

COPY --chmod=755 install_alpine_deps /tmp
RUN . /tmp/install_alpine_deps

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# PATH=/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
# GOPATH=/go
RUN cd "$TMOE_DIR"; \
    printf "%s\n" \
    'export PATH="/go/bin:/usr/local/go/bin${PATH:+:${PATH}}"' \
    'export GOPATH="/go"' \
    > environment/container.env; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "go = '$(go version)'" \
    "gofmt = '$(go version $(command -v gofmt))'" \
    "" \
    '[other]' \
    'workdir = "/go"' \
    > version.toml; \
    cat version.toml

# clean: apk -v cache clean
RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    2>/dev/null

CMD [ "/bin/bash" ]
