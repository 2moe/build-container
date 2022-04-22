
# syntax=docker/dockerfile:1
#---------------------------
# cblmariner.azurecr.io/base/core:1.0
FROM cblmariner2preview.azurecr.io/base/core:2.0

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

RUN yes | yum install -y dnf
RUN yes | dnf install -y --skip-broken sudo tar xz newt glibc-all-langpacks passwd shadow-utils hostname glibc-lang glibc-i18n
RUN mkdir -p /run/dbus

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN mkdir -p $TMOE_DIR/environment \
    && cd $TMOE_DIR \
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