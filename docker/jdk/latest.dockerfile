FROM --platform=${TARGETPLATFORM}  openjdk:jdk-slim
ENV HOME=/root \
    TMOE_PROOT=false \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true

RUN apt update; \
    apt dist-upgrade -y; \
    dpkg --configure -a; \
    apt install -y sudo locales curl; \
    apt install -y whiptail eatmydata procps apt-utils; \
    apt install -y --no-install-recommends neofetch; \
    printf "%s\n" "root:root" | chpasswd; \
    mkdir -p /run/systemd; \
    printf "%s\n" "docker" > /run/systemd/container; \
    mkdir -pv /usr/local/etc/tmoe-linux; \
    cd /usr/local/etc/tmoe-linux; \
    printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=openjdk_nogui-debian" \
    > container.txt ; \
    mkdir -p environment; \
    JAVA_DIR=$(command -v java); \
    JAVA_PATH=${JAVA_DIR%/*}; \
    printf "%s\n" \
    "export PATH=\"${JAVA_PATH}\${PATH:+:\${PATH}}\"" \
    "export JAVA_HOME=${JAVA_HOME}" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "jshell" \
    > environment/entrypoint; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "JAVA_VERSION='$(java --version)'" \
    "JAVAC_VERSION='$(javac --version)'" \
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
CMD [ "jshell" ]
