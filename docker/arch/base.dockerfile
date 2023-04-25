# syntax=docker/dockerfile:1
#---------------------------
# DOCKER_BUILDKIT=1 docker build -t u2 -f ../arch/base.dockerfile --platform=linux/amd64 .
# --build-arg ARCH=amd64

FROM amd64/alpine 
# AS get_arch_rootfs
ARG URL="https://github.com/2moe/build-container/releases/download/v0.0.0-alpha.2/get-lxc_0.0.0.alpha.2_amd64.deb"

# install curl
RUN apk add dpkg curl

WORKDIR /tmp
# install deb
RUN curl -Lo get-lxc.deb "${URL}" \
    && dpkg -i --force-architecture ./get-lxc.deb

# get arch, get url & download file
ARG TARGETARCH
ARG TARGETVARIANT
COPY --chmod=755 get_arch /tmp
RUN . ./get_arch \
    && get-lxc -o arch -c current --var default -a $DEB_ARCH --src gh -m us -t 2 -d . -f arch.tar.xz

RUN mv arch.tar.xz /

CMD ["/bin/sh"]
