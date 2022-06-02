# syntax=docker/dockerfile:1
#---------------------------
FROM --platform=${TARGETPLATFORM} fedora:rawhide

WORKDIR /root
ARG DNF_RC=/etc/dnf/dnf.conf
ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux"

# modify dnf conf
RUN sed -E \
    -e 's@^(gpgcheck)=.*@\1=0@g' \
    -e '$a\fastestmirror=True' \
    -e '$a\max_parallel_downloads=3' \
    -i "${DNF_RC}"; \
    cat "${DNF_RC}"

# install dependencies
RUN yes | dnf update -y; \
    dnf install -y \
    --skip-broken \
    glibc-all-langpacks \
    glibc-minimal-langpack \
    glibc-locale-source
# glibc-langpack-en

# set locale
COPY --chmod=755 set_locale /tmp
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

RUN dnf install -y \
    --skip-broken \
    iproute \
    lolcat \
    newt \
    systemd \
    dnf-utils \
    passwd \
    findutils \
    man-db \
    procps-ng \
    procps-ng-i18n \
    tar \
    hostname \
    neofetch \
    aria2 \
    zstd \
    curl \
    wget

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# configure zsh
COPY --chmod=755 configure_zsh /tmp
RUN . /tmp/configure_zsh

# clean
RUN rm -rfv \
    ~/.cache/* \
    /tmp/* \
    2>/dev/null
RUN dnf clean all

CMD [ "zsh" ]
