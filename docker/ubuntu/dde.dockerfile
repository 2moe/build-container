# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} ubuntu:21.10

WORKDIR /root
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt install -y software-properties-common

RUN apt install -y \
    locales-all \
    wget \
    dialog \
    aria2 \
    zstd \
    systemd \
    aptitude

RUN apt install -y \
    --no-install-recommends \
    lolcat \
    unzip

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# configure zsh
COPY --chmod=755 configure_zsh /tmp
RUN . /tmp/configure_zsh

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv /tmp/* 2>/dev/null

ARG DEBIAN_FRONTEND=noninteractive
ENV LANG="en_US.UTF-8"

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN . /tmp/gen_tool

ARG AUTO_INSTALL_GUI=true
RUN bash /tmp/install-gui.sh

WORKDIR /root

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv \
    ~/.vnc/*passwd \
    /tmp/* \
    2>/dev/null

EXPOSE 5902 36080

CMD ["zsh"]
