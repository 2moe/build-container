# syntax=docker/dockerfile:1
FROM cake233/alpine-zsh-${TARGETARCH}${TARGETVARIANT}
# :zstd as rootfs

RUN apk add zstd tar

RUN mkdir -p /rootfs \
    && tar -xvf /root/rootfs.tar.zst -C /rootfs


# FROM scratch
# COPY --from=rootfs /rootfs /

ARG TAG
ARG OS

RUN cd ${TMOE_DIR} \
    && printf "%s\n" \
    "CONTAINER_TYPE=podman" \
    "CONTAINER_NAME=alpine_gui-${ARG}" \
    > container.txt \
    && mv /root/docker_tool /tmp  

WORKDIR /tmp
RUN sed -i 's@*) #main@2333)@g' docker_tool; \
    printf "%s\n" \
    "source /tmp/docker_tool -install-deps"  \
    "cd /usr/local/etc/tmoe-linux/git/share/old-version/tools/gui/" \
    "source gui --auto-install-gui-${TAG}" \
    > install-gui.sh
ARG AUTO_INSTALL_GUI=true
RUN bash install-gui.sh

RUN rm -rf \
    /var/cache/apk/* \
    ~/.cache/* \
    /tmp/* \
    ~/.vnc/*passwd