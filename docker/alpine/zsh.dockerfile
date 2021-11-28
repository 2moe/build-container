# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} alpine:edge

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="C.UTF-8"

# dependencies
COPY --chmod=755 install_alpine_deps /tmp
RUN . /tmp/install_alpine_deps

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# configure zsh
COPY --chmod=755 configure_zsh /tmp
RUN . /tmp/configure_zsh

CMD [ "/bin/zsh" ]
