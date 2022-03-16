# syntax=docker/dockerfile:1
#---------------------------
ARG SWIFT_PLATFORM=ubuntu
ARG OS_MAJOR_VER=20
ARG OS_MIN_VER=04

FROM --platform=${TARGETPLATFORM} ${SWIFT_PLATFORM}:${OS_MAJOR_VER}.${OS_MIN_VER}

# twice
ARG SWIFT_PLATFORM=ubuntu
ARG OS_MAJOR_VER=20
ARG OS_MIN_VER=04

ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt-get install -y \
    locales-all \
    nano

RUN apt-get install -y \
    binutils \
    git \
    gnupg2 \
    libc6-dev \
    libcurl4-openssl-dev \
    libedit2 \
    libgcc-9-dev \
    libpython3.8 \
    libsqlite3-0 \
    libstdc++-9-dev \
    libxml2-dev \
    libz3-dev \
    pkg-config \
    tzdata \
    zlib1g-dev

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8


ARG SWIFT_WEBROOT=https://download.swift.org/development

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
COPY --chmod=755 swift/download_swift /tmp
RUN . /tmp/set_container_txt \
    && . /tmp/download_swift

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "swift = '''" \
    "$(swift --version)" \
    "'''" \
    > version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "bash" ]
