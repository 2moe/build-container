# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} ruby:latest

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
    'export PATH="/usr/local/bundle/bin${PATH:+:${PATH}}"' \
    "export GEM_HOME='/usr/local/bundle'" \
    "export BUNDLE_SILENCE_ROOT_WARNING='1'"\
    "export BUNDLE_APP_CONFIG='/usr/local/bundle'" \
    > environment/container.env; \
    printf "%s\n" \
    'cd ~' \
    "irb" \
    > environment/entrypoint; \
    chmod -R a+rx environment/

RUN gem install webrick

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ruby = '$(ruby --version)'" \
    "gem = '$(gem --version)'" \
    "bundle = '$(bundle --version)'" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "irb" ]
