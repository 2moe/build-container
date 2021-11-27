FROM --platform=${TARGETPLATFORM} mongo:latest

WORKDIR /root
ARG DEBIAN_FRONTEND=noninteractive
ENV HOME=/root \
    TMOE_PROOT=false \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true

RUN apt update; \
    apt dist-upgrade -y; \
    dpkg --configure -a; \
    apt install -y sudo locales locales-all; \
    apt install -y whiptail curl eatmydata apt-utils procps; \
    apt install -y --no-install-recommends neofetch; \
    printf "%s\n" "root:root" | chpasswd; \
    mkdir -p /run/systemd; \
    printf "%s\n" "docker" > /run/systemd/container; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=mongo_nogui-ubuntu" \
    > container.txt ; \
    mkdir -p environment; \
    printf "%s\n" \
    "export MONGO_PACKAGE=${MONGO_PACKAGE}" \
    "export MONGO_REPO=${MONGO_REPO}" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "mongod" \
    > environment/entrypoint; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "GOSU_VERSION='$(gosu --version)'"  \
    "JSYAML_VERSION='${JSYAML_VERSION}'" \
    "MONGO_MAJOR='${MONGO_MAJOR}'"  \
    "MONGO_VERSION='${MONGO_VERSION}'" \
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

CMD [ "mongod" ]
