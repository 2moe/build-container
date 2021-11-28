# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} manjarolinux/base:latest

WORKDIR /root
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

ARG TARGETARCH
COPY --chmod=755 arch_key /tmp
# SHELL [ "bash", "-cex" ]
RUN . /tmp/arch_key

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

WORKDIR /root

# base-devel
RUN pacman -Syyu --needed --noconfirm base base-devel

# install dependencies
RUN pacman \
    -S \
    --noconfirm \
    --needed \
    git \
    unzip \
    neofetch \
    iproute \
    zsh \
    wget \
    curl

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# configure zsh
COPY --chmod=755 configure_zsh /tmp
RUN . /tmp/configure_zsh

# WORKDIR /tmp
# add archlinux mirror repo & install fakeroot-tcp
RUN cd /tmp; \
    cp -fv "${TMOE_DIR}"/git/share/old-version/tools/sources/yay/build_fakeroot ./; \
    chmod a+rx -v build_fakeroot; \
    ./build_fakeroot --add-arch_for_edu-repo; \
    ./build_fakeroot --add-archlinuxcn-repo; \
    ./build_fakeroot --install-yay; \
    ./build_fakeroot --install-fakeroot; \
    ./build_fakeroot --archlinux-repo-mirror; \
    cat /etc/pacman.conf

# clean
RUN rm -rfv \
    /var/cache/pacman/pkg/* \
    ~/.cache/* \
    /tmp/*  \
    2>/dev/null; \
    yes | pacman -Scc

CMD ["/usr/bin/zsh"]

