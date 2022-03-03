FROM cake233/ubuntu-kde-${TARGETARCH}${TARGETVARIANT} AS builder

ARG DEBIAN_FRONTEND=noninteractive
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

WORKDIR /root

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 install_cutefish /tmp
RUN . /tmp/install_cutefish

RUN cd ~/cutefish \
    && mkdir -pv ~/deb \
    && mv -vf *.deb *.buildinfo *.changes *.deb ~/deb

# clean
COPY --chmod=755 clean_deb_cache /tmp
RUN . /tmp/clean_deb_cache
RUN rm -rfv \
    ~/.vnc/*passwd \
    /tmp/* \
    2>/dev/null

EXPOSE 5902 36080

FROM ubuntu:devel
WORKDIR /root
COPY --from=builder /root/deb .

CMD ["bash"]

