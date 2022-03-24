# syntax=docker/dockerfile:1
#---------------------------
# FROM cake233/kali-zsh-${TARGETARCH}${TARGETVARIANT}
FROM scratch
ADD rootfs.tar /

ARG DEBIAN_FRONTEND=noninteractive
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
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv \
    ~/.vnc/*passwd \
    /tmp/* \
    2>/dev/null

EXPOSE 5902 36080

CMD ["zsh"]