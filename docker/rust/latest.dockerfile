# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} debian:unstable

WORKDIR /root
# set arg & env
ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH"

# install dependencies
COPY --chmod=755 install_deb_deps /tmp
RUN . /tmp/install_deb_deps

RUN apt install -y \
    pkg-config \
    libssl-dev

RUN apt install \
    -y \
    --no-install-recommends \
    gcc \
    libc6-dev

# minimal, default, complete
ARG RUSTUP_PROFILE=default
ARG GNU_TARGET
# ARG MUSL_TARGET
# x86_64-unknown-linux-gnu/rustup-init
RUN export RUSTUP_URL="https://static.rust-lang.org/rustup/dist/${GNU_TARGET}/rustup-init"; \
    curl -LO ${RUSTUP_URL} || exit 1; \
    chmod +x rustup-init \
    && ./rustup-init \
    -y \
    --profile ${RUSTUP_PROFILE} \
    --no-modify-path \
    --default-toolchain \
    nightly \
    && rm rustup-init \
    && chmod -Rv a+w ${RUSTUP_HOME} ${CARGO_HOME}

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
    'export PATH="/usr/local/cargo/bin${PATH:+:${PATH}}"' \
    'export RUSTUP_HOME="/usr/local/rustup"' \
    'export CARGO_HOME="/usr/local/cargo"' \
    > environment/container.env; \
    chmod -R a+rx environment/

RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "rustup = '$(rustup --version)'" \
    "cargo = '$(cargo --version)'" \
    "rustc = '$(rustc --version)'" \
    "cc = '$(cc --version | head -n 1)'" \
    "cargo_verbose = '''" \
    "$(cargo -Vv)" \
    "'''" \
    "rustc_verbose = '''" \
    "$(rustc -Vv)" \
    "'''" \
    > version.toml; \
    cat version.toml

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache

CMD [ "/usr/bin/bash" ]
