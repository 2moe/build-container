# syntax=docker/dockerfile:1
#---------------------------
# FROM cake233/fedora-zsh-${TARGETARCH}${TARGETVARIANT}
FROM scratch
ADD rootfs.tar /

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN . /tmp/gen_tool

ARG AUTO_INSTALL_GUI=true
RUN bash /tmp/install-gui.sh

WORKDIR /root

# clean
RUN rm -rfv \
    ~/.vnc/*passwd \
    ~/.cache/* \
    /tmp/* \
    2>/dev/null
RUN dnf clean all

EXPOSE 5902 36080
CMD ["/usr/bin/zsh"]
