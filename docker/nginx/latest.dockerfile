# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} nginx:latest
# WORKDIR /root
ADD nginx.txt /root/readme.txt

ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

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

RUN cd "$TMOE_DIR"; \
    printf "%s\n" \
    'cd ~' \
    "nginx -g 'daemon off;'" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version | head -n 1)'" \
    "nginx = '${NGINX_VERSION}'"  \
    "njs = '${NJS_VERSION}'"  \
    "pkg_release = '${PKG_RELEASE}'" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD ["nginx", "-g", "daemon off;"]
