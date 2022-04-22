# syntax=docker/dockerfile:1
#---------------------------
FROM openeuler/openeuler

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

RUN yes | dnf install -y sudo tar xz newt

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN mkdir -p $TMOE_DIR/environment
RUN . /tmp/gen_tool

WORKDIR /root

# clean
RUN rm -rfv \
    ~/.cache/* \
    /tmp/* \
    2>/dev/null
RUN dnf clean all

CMD ["bash"]