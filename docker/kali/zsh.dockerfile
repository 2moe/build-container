# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} kalilinux/kali-rolling:latest

WORKDIR /root
# ARG EXP_LIST="/etc/apt/sources.list.d/kali-experimental.list"
ARG DEBIAN_FRONTEND=noninteractive
ARG EXP_LIST="/etc/apt/sources.list.d/experimental.list"
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt install -y \
    locales-all \
    wget \
    dialog \
    aria2 \
    zstd \
    systemd \
    aptitude \
    whiptail

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

# RUN mv -v ${EXP_LIST} ${EXP_LIST}.bak

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv /tmp/* 2>/dev/null

CMD ["zsh"]
