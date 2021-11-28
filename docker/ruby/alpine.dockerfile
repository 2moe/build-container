# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} ruby:alpine

# ENV PATH=/usr/local/bundle/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
#     GEM_HOME=/usr/local/bundle \
#     BUNDLE_SILENCE_ROOT_WARNING=1 \
#     BUNDLE_APP_CONFIG=/usr/local/bundle
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

# clean: apk -v cache clean
RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    2>/dev/null 

CMD [ "irb" ]
