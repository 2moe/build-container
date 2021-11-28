# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} openjdk:jdk-slim

WORKDIR /root
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

RUN cd ${TMOE_DIR}; \
    JAVA_DIR=$(command -v java); \
    JAVA_PATH=${JAVA_DIR%/*}; \
    printf "%s\n" \
    "export PATH=\"${JAVA_PATH}\${PATH:+:\${PATH}}\"" \
    "export JAVA_HOME='${JAVA_HOME}'" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "jshell" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "java = '''" \
    "$(java --version)" \
    "'''" \
    "javac = '$(javac --version)'" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
# RUN rm -rfv /tmp/* 2>/dev/null

CMD [ "jshell" ]
