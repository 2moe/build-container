# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} node:alpine
COPY node.js /root/readme.js

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
    "node" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "node = '$(node --version)'" \
    "yarn = '$(yarn --version)'" \
    "npm = '$(npm --version)'" \
    > version.toml; \
    cat version.toml

# clean: apk -v cache clean
RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    2>/dev/null

CMD [ "node" ]
