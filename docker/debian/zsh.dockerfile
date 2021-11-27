# syntax=docker/dockerfile:1

FROM --platform=${TARGETPLATFORM} debian:experimental

ENV TMOE_CHROOT true \
    TMOE_DOCKER true \
    TMOE_DIR "/usr/local/etc/tmoe-linux"

WORKDIR /root
ARG DEBIAN_FRONTEND=noninteractive
ARG EXP_LIST="/etc/apt/sources.list.d/experimental.list"

RUN apt update; \
    apt dist-upgrade -y; \
    dpkg --configure -a; \
    apt install -y locales-all locales; \
    apt install -y whiptail dialog aria2 zstd curl wget; \
    apt install -y --no-install-recommends neofetch lolcat unzip; \
    apt install -y apt-utils systemd procps aptitude; \
    printf "%s\n" "root:root" | chpasswd; \
    mkdir -p /run/systemd; \
    printf "%s\n" "docker" > /run/systemd/container; \
    cd /tmp; \
    curl -LO 'https://github.com/2cd/zsh/raw/master/zsh.sh' || exit 1; \
    bash zsh.sh --tmoe_container_auto_configure; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=debian_nogui-zsh" \
    > container.txt ; \
    git clone -b master --depth=1 https://github.com/2moe/tmoe-linux git || git clone -b master --depth=1 git://github.com/2moe/tmoe-linux git; \
    cp -fv git/share/old-version/tools/app/tool /root/docker_tool ; \
    cd /tmp; \
    rm -rfv /tmp/* ~/.vnc/*passwd ~/.cache/* 2>/dev/null; \
    apt autopurge -y; \
    mv -v ${EXP_LIST} ${EXP_LIST}.bak; \
    rm -fv /var/cache/apt/archives/* \
    /var/cache/apt/* \
    /var/mail/* \
    /var/log/* \
    /var/log/apt/* \
    /var/log/journal/* \
    /var/lib/apt/lists/* \
    2>/dev/null; \
    apt clean

CMD ["/usr/bin/zsh"]
