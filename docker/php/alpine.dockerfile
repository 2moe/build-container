FROM --platform=${TARGETPLATFORM} php:alpine
WORKDIR /root
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
    "CONTAINER_NAME=php_nogui-alpine" \
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
    rm -rf /var/cache/apk/* ~/.cache/* 2>/dev/null 
#apk -v cache clean

CMD [ "php","-a" ]
