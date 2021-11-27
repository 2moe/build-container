# syntax=docker/dockerfile:1
FROM --platform=${TARGETPLATFORM} alpine:edge

ARG URL="https://github.com/2cd/zsh/raw/master/zsh.sh"
ENV TMOE_CHROOT true \
    TMOE_DOCKER true \
    TMOE_DIR "/usr/local/etc/tmoe-linux"

# upgrade pkgs
RUN apk update \
    && apk upgrade

# install deps
RUN apk add \
    tar \
    grep \
    curl \
    wget \
    zstd \
    bash \
    tzdata \
    newt \
    shadow \
    git \
    zsh

# password: root, timezone: UTC
RUN printf "%s\n" \
    "root:root" |\
    chpasswd; \
    ln -svf /usr/share/zoneinfo/UTC /etc/localtime

# configure zsh
RUN cd /tmp || cd ~; \
    curl -LO "$URL" || exit 1 \
    && bash zsh.sh --tmoe_container_auto_configure

ARG URL="https://github.com/2moe/tmoe-linux"
# set configuration 
RUN mkdir -p ${TMOE_DIR}; \
    cd ${TMOE_DIR} || exit 9 \
    && printf "%s\n" \
    "CONTAINER_TYPE=docker" \
    "CONTAINER_NAME=alpine_nogui-zsh" \
    > container.txt; \
    git clone \
    -b master \
    --depth=1 \
    "$URL" git \
    && cp -fv git/share/old-version/tools/app/tool /root/docker_tool

CMD [ "zsh" ]
