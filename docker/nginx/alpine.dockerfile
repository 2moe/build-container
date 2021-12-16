# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} nginx:alpine

# WORKDIR /root
ADD nginx.txt /root/readme.txt

ENV LANG="C.UTF-8" \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

COPY --chmod=755 install_alpine_deps /tmp
RUN . /tmp/install_alpine_deps

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
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "nginx = '${NGINX_VERSION}'"  \
    "njs = '${NJS_VERSION}'"  \
    "pkg_release = '${PKG_RELEASE}'" \
    > version.toml; \
    cat version.toml

# clean: apk -v cache clean
RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    2>/dev/null

CMD ["nginx", "-g", "daemon off;"]
