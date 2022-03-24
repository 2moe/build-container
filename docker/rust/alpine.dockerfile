# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} alpine:edge

WORKDIR /root
# PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV LANG="C.UTF-8" \
    TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH"

# install dependencies
COPY --chmod=755 install_alpine_deps /tmp
RUN . /tmp/install_alpine_deps

# install musl-dev
RUN apk add openssl-dev \
    musl-dev \
    gcc \
    ca-certificates

# minimal, default, complete
ARG RUSTUP_PROFILE=minimal
ARG MUSL_TARGET
RUN export RUSTUP_URL="https://static.rust-lang.org/rustup/dist/${MUSL_TARGET}/rustup-init"; \
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
# RUN rustup update

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# export env to file
RUN cd ${TMOE_DIR}; \
    printf "%s\n" \
    'export PATH="/usr/local/cargo/bin${PATH:+:${PATH}}"' \
    'export RUSTUP_HOME="/usr/local/rustup"' \
    'export CARGO_HOME="/usr/local/cargo"' \
    > environment/container.env; \
    chmod -R a+rx environment/

# export version info to file
RUN cd /root; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
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

# clean: apk -v cache clean
RUN rm -rf /var/cache/apk/* \
    ~/.cache/* \
    2>/dev/null 

CMD ["bash"]
