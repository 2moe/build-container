# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} php:latest

# set arg & env
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# export env to file
RUN cd "$TMOE_DIR"; \
    printf "%s\n" \
    "export PHP_INI_DIR='${PHP_INI_DIR}'" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "php -a" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version | head -n 1)'" \
    "php = '''" \
    "$(php --version)" \
    "'''" \
    "" \
    '[other]' \
    "phpize_deps = '${PHPIZE_DEPS}'" \
    "php_extra_configure_args = '--enable-embed'"  \
    "php_cflags = '${PHP_CFLAGS}'" \
    "php_cppflags = '${PHP_CPPFLAGS}'" \
    "php_ldflags = '${PHP_LDFLAGS}'"  \
    "gpg_keys = '${GPG_KEYS}'" \
    "php_url = '${PHP_URL}'"  \
    "php_src_url = '${PHP_ASC_URL}'" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "php","-a" ]
