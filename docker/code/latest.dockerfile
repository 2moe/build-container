# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} debian:unstable-slim

ADD code.js /root/readme.js
COPY --chmod=777 code/init /root
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

# no-install-recommends
RUN apt install \
    -y \
    --no-install-recommends \
    nano \
    bat

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

# batcat -> bat
RUN if [ -n "$(command -v batcat)" ];then ln -svf $(command -v batcat) /usr/bin/bat;fi

ARG ARCH
# install code
RUN cd /tmp; \
    THE_LATEST_LINK=$(curl -L https://api.github.com/repos/cdr/code-server/releases | grep "${ARCH}" | grep browser_download_url | grep \.deb | head -n 1 | awk -F ' ' '$0=$NF' | cut -d '"' -f 2); \
    THE_LATEST_LINK=$(echo $THE_LATEST_LINK | sed -E -e "s@v(4.0).2/@v\1.1/@" -e "s@(_4.0).2_@\1.1_@"); \
    curl -Lo code.deb ${THE_LATEST_LINK} || exit 1; \
    apt install -y ./code.deb; \
    rm -fv code.deb

RUN apt install -y libatomic1 2>/dev/null

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

RUN cd "$TMOE_DIR"; \
    printf "%s\n" \
    'cd ~' \
    "/usr/local/bin/code" \
    > environment/entrypoint; \
    chmod -v a+rx environment/*

# bin/code
RUN printf "%s\n" \
    '#!/usr/bin/env bash' \
    'code-server $@ &' \
    "bat -ppnl yaml ~/.config/code-server/config.yaml || cat ~/.config/code-server/config.yaml" \
    'printf "You can type \033[32m%s\033[m or \033[32m%s\033[m to start it, type \033[32m%s\033[m to \033[31m%s\033[m.\n" "code" "code-server" "pkill node" "stop"' \
    'printf "You can also type code \$FILE_DIR to create a new window and open the specified directory.\n"' \
    > /usr/local/bin/code; \
    chmod a+rx /usr/local/bin/code

# export version info to file
RUN cd /root; \
    code-server --version; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "code = '$(code-server --version)'" \
    "" \
    '[port]' \
    "tcp = [8080]" \
    > version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

EXPOSE 8080

CMD [ "code-server" ]
