# syntax=docker/dockerfile:1
#---------------------------
# docker pull swiftlang/swift:nightly-focal
FROM --platform=${TARGETPLATFORM} swiftlang/swift:nightly-5.6-focal

ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt install -y \
    locales-all \
    nano \
    &&  rm /etc/motd 2>/dev/null

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "swift = ''' \
    "$(swift --version)" \
    ''' \
    "" \
    > version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "bash" ]
