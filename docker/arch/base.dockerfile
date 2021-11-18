# syntax=docker/dockerfile:1

# DOCKER_BUILDKIT=1 docker build -t u2 -f ./arch/base.dockerfile --platform=linux/arm64 .
# --build-arg ARCH amd64
# docker build -t u2 -f arch/base.dockerfile --build-arg TARGETARCH=amd64 .

FROM amd64/alpine AS get_arch_rootfs
ARG URL="https://github.com/2moe/build-container/releases/download/0.0.1-alpha/get-arch-url_0.0.1_amd64.deb"

# install curl
RUN apk add dpkg curl

WORKDIR /root
# install deb
RUN curl -Lo get-url.deb "${URL}" \
    && dpkg -i --force-architecture ./get-url.deb

# get arch, get url & download file
ARG TARGETARCH
ARG TARGETVARIANT
COPY --chmod=755 get_arch /tmp
RUN . /tmp/get_arch \
    && echo $ARCH \
    && get-arch-url \
    && new_url=$(cat url.txt) \
    && curl -Lo arch.tar.xz "$new_url"

# extract xz
# RUN xz -dv arch.tar.xz
RUN mkdir /arch \
    && tar -Jxvf arch.tar.xz -C /arch

FROM scratch
COPY --from=get_arch_rootfs /arch /

RUN pacman-key --init
ARG TARGETARCH
COPY --chmod=755 arch_key /tmp
RUN . /tmp/arch_key

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

# clean
RUN yes | pacman -Scc \
    && rm -rfv /var/lib/pacman/*

CMD ["bash"]
