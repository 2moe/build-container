FROM --platform=${TARGETPLATFORM}  node:alpine

ADD README.js /root
ENV HOME=/root \
    TMOE_PROOT=false \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true
RUN apk update; \
    apk upgrade; \
    apk add sudo tar grep curl wget bash tzdata newt shadow; \
    printf "%s\n" "root:root" | chpasswd; \
    ln -svf /usr/share/zoneinfo/UTC /etc/localtime; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux/; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=node_nogui-alpine" \
    > container.txt ; \
    mkdir -p environment; \
    printf "%s\n" \
    'cd ~' \
    "node" \
    > environment/entrypoint; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "NODE_VERSION='$(node --version)'" \
    "YARN_VERSION='$(yarn --version)'" \
    "NPM_VERSION='$(npm --version)'" \
    > version.txt; \
    rm -rf /var/cache/apk/* ~/.cache/* 2>/dev/null
#apk -v cache clean
#npm i -g npm

CMD [ "node" ]
