# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} mongo:latest

# WORKDIR /root
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt install -y locales-all

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
RUN cd ${TMOE_DIR}; \
    printf "%s\n" \
    "export MONGO_PACKAGE='${MONGO_PACKAGE}'" \
    "export MONGO_REPO='${MONGO_REPO}'" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "mongod" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "gosu = '$(gosu --version)'"  \
    "jsyaml = '${JSYAML_VERSION}'" \
    "mongo_major = '${MONGO_MAJOR}'"  \
    "mongo_version = '${MONGO_VERSION}'" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "mongod" ]
