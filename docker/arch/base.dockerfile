# syntax=docker/dockerfile:1

# DOCKER_BUILDKIT=1 docker build -t u2 -f ../arch/base.dockerfile --platform=linux/amd64 .
# --build-arg ARCH=amd64

FROM amd64/alpine 
# AS get_arch_rootfs
ARG URL="https://github.com/2moe/build-container/releases/download/0.0.1-alpha/get-arch-url_0.0.1_amd64.deb"

# install curl
RUN apk add dpkg curl

WORKDIR /tmp
# install deb
RUN curl -Lo get-url.deb "${URL}" \
    && dpkg -i --force-architecture ./get-url.deb

# get arch, get url & download file
ARG TARGETARCH
ARG TARGETVARIANT
COPY --chmod=755 get_arch /tmp
RUN . ./get_arch \
    && get-arch-url \
    && curl -Lo arch.tar.xz "$(cat url.txt)"

RUN mv arch.tar.xz /

CMD ["sh"]
