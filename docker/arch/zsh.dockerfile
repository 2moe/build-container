# syntax=docker/dockerfile:1

FROM cake233/arch-${TARGETARCH}${TARGETVARIANT}

# ARG OS=arch
# ARG RELEASE=latest
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# install base-devel
RUN pacman -Syu --noconfirm --needed base base-devel

# install deps
RUN pacman \
    -S \
    --noconfirm \
    --needed \
    git \
    unzip \
    neofetch \
    iproute

# set password: root
RUN printf "%s\n" \
    "root:root" | \ 
    chpasswd

# container: docker
RUN mkdir -pv /run/systemd \
    && echo "docker" > /run/systemd/container

ARG TAG=zsh
ARG TMOE_GIT_URL="https://github.com/2moe/tmoe-linux"
ARG ARCH
ARG OS
RUN mkdir -p ${TMOE_DIR} \
    && cd ${TMOE_DIR} \
    && printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=${OS}_nogui-${TAG}" \
    "ARCH=${ARCH}" \
    > container.txt; \
    git clone \
    -b master \
    --depth=1 \
    ${TMOE_GIT_URL} \
    git \
    && cp -fv git/share/old-version/tools/sources/yay/build_fakeroot /tmp

WORKDIR /tmp
ARG URL="https://github.com/2cd/zsh/raw/master/zsh.sh"
RUN curl -LO "$URL" || exit 1 \
    && bash zsh.sh --tmoe_container_auto_configure

# add archlinux mirror repo & install fakeroot-tcp
RUN chmod a+rx -v build_fakeroot; \
    ./build_fakeroot --add-arch_for_edu-repo; \
    ./build_fakeroot --add-archlinuxcn-repo; \
    ./build_fakeroot --install-yay; \
    ./build_fakeroot --install-fakeroot; \
    ./build_fakeroot --archlinux-repo-mirror; \
    cat /etc/pacman.conf

WORKDIR /root

# clean
RUN rm -rfv \
    /var/cache/pacman/pkg/* \
    ~/.cache/* \
    /tmp/*  \
    2>/dev/null; \
    yes | pacman -Scc

CMD ["/usr/bin/zsh"]
