FROM --platform=${TARGETPLATFORM}  nginx:latest
WORKDIR /root
ADD nginx.txt /root/README.txt

ARG DEBIAN_FRONTEND=noninteractive
ENV HOME=/root \
    TMOE_PROOT=false \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true

RUN apt update; \
    apt dist-upgrade -y; \
    dpkg --configure -a; \
    apt install -y sudo locales; \
    apt install -y whiptail curl eatmydata apt-utils procps; \
    apt install -y --no-install-recommends neofetch; \
    printf "%s\n" "root:root" | chpasswd; \
    mkdir -p /run/systemd; \
    printf "%s\n" "docker" > /run/systemd/container; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=nginx_nogui-debian" \
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
    rm -fv /var/cache/apt/archives/* \
    /var/cache/apt/* \
    /var/mail/* \
    /var/log/* \
    /var/log/apt/* \
    /var/log/journal/* \
    /var/lib/apt/lists/* \
    2>/dev/null; \
    apt clean

CMD ["nginx", "-g", "daemon off;"]
