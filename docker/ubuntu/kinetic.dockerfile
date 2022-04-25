# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} ubuntu:devel

WORKDIR /root
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

COPY --chmod=755 ubuntu/devel /tmp
RUN bash /tmp/devel

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

# RUN apt install -y software-properties-common

RUN apt install -y \
    locales-all \
    systemd


# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv /tmp/* 2>/dev/null

CMD ["bash"]