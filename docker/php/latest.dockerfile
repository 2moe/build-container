FROM --platform=${TARGETPLATFORM} php:latest
WORKDIR /root
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
    "CONTAINER_NAME=php_nogui-debian" \
    > container.txt ; \
    mkdir -p environment; \
    printf "%s\n" \
    "export PHP_INI_DIR=${PHP_INI_DIR}" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "php -a" \
    > environment/entrypoint; \
    chmod -R a+rx environment/; \
    cd /root; \
    printf "%s\n" \
    "PHP_VERSION='$(php --version)'" \
    "PHPIZE_DEPS='${PHPIZE_DEPS}'" \
    "PHP_EXTRA_CONFIGURE_ARGS='--enable-embed'"  \
    "PHP_CFLAGS='${PHP_CFLAGS}'" \
    "PHP_CPPFLAGS='${PHP_CPPFLAGS}'" \
    "PHP_LDFLAGS='${PHP_LDFLAGS}'"  \
    "GPG_KEYS='${GPG_KEYS}'" \
    "PHP_URL='${PHP_URL}'"  \
    "PHP_ASC_URL='${PHP_ASC_URL}'" \
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

CMD [ "php","-a" ]
