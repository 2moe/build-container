FROM --platform=${TARGETPLATFORM}  nginx:alpine
WORKDIR /root
ADD nginx.txt /root/README.txt

WORKDIR /root
ADD README.txt /root
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
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=nginx_nogui-alpine" \
    > container.txt ; \
    mkdir -p environment; \
    printf "%s\n" \
    'cd ~' \
    "nginx -g 'daemon off;'" \
    > environment/entrypoint; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "NGINX_VERSION='${NGINX_VERSION}'"  \
    "NJS_VERSION='${NJS_VERSION}'"  \
    "PKG_RELEASE='${PKG_RELEASE}'" \
    > version.txt; \
    cat version.txt; \
    rm -rf /var/cache/apk/* ~/.cache/* 2>/dev/null 
#apk -v cache clean

CMD ["nginx", "-g", "daemon off;"]
