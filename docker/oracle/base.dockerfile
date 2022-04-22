# syntax=docker/dockerfile:1
#---------------------------
FROM oraclelinux:8

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

RUN if [ -z $(command -v dnf) ];then ln -svf $(command -v yum) /usr/bin/dnf; fi

RUN yes | dnf install -y --skip-broken sudo tar xz newt glibc-all-langpacks passwd
RUN mkdir -p /run/dbus

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN mkdir -p $TMOE_DIR/environment \
    && printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=${OS}_nogui-${TAG}" \
    "ARCH=${ARCH}" \
    >container.txt

WORKDIR /root

# clean
RUN rm -rfv \
    ~/.cache/* \
    /tmp/* \
    2>/dev/null
RUN dnf clean all

CMD ["bash"]