# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} golang:latest
COPY go.go /root/README.go

# set workdir & env
WORKDIR /go
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# PATH=/go/bin:/usr/local/go/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
# GOPATH=/go
# export env to file
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

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD ["bash"]