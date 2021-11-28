# syntax=docker/dockerfile:1
#---------------------------
# FROM cake233/alpine-zsh-${TARGETARCH}${TARGETVARIANT}
FROM scratch
ADD rootfs.tar /

ENV LANG="C.UTF-8" \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

RUN apk add zstd

ARG TAG
ARG OS
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN . /tmp/gen_tool

# auto install gui
ARG AUTO_INSTALL_GUI=true
RUN bash /tmp/install-gui.sh

# clean
RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    /tmp/* \
    ~/.vnc/*passwd

# expose tcp ports
EXPOSE 5902 36080

# command: zsh
CMD ["/bin/zsh"]
