# syntax=docker/dockerfile:1

# FROM cake233/arch-zsh-${TARGETARCH}${TARGETVARIANT}
FROM scratch

ADD rootfs.tar /
WORKDIR /tmp

RUN pacman -Syu --noconfirm --needed \
    man-db \
    man-pages

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool .
RUN . ./gen_tool

# auto install gui
ARG AUTO_INSTALL_GUI=true
RUN bash install-gui.sh

WORKDIR /root
# remove -Qdtq
RUN pacman -R \
    --noconfirm \
    $(pacman -Qdtq); \
    rm -rfv ~/.vnc/*passwd 2>/dev/null

# clean /var/cache/pacman/pkg/
RUN rm -rfv \
    ~/.cache/* \
    /tmp/*  \
    2>/dev/null; \
    yes | pacman -Scc

# expose tcp ports
EXPOSE 5902 36080

# command: zsh
CMD ["/usr/bin/zsh"]
