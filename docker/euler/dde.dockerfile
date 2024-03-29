# syntax=docker/dockerfile:1
#---------------------------
# FROM cake233/fedora-zsh-${TARGETARCH}${TARGETVARIANT}
FROM cake233/euler-${TARGETARCH}${TARGETVARIANT}

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    LANG="en_US.UTF-8"

RUN yes | dnf install -y git zsh

ARG URL="https://github.com/2moe/tmoe-linux"
# set configuration
RUN mkdir -p "${TMOE_DIR}" \
    && cd "${TMOE_DIR}"  \
    && git clone \
    -b master \
    --depth=1 \
    "$URL" git

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 gen_tool /tmp
RUN . /tmp/gen_tool

# ARG AUTO_INSTALL_GUI=true
# RUN bash /tmp/install-gui.sh
RUN yes | dnf install -y --skip-broken dde || echo "failed to install dde"
RUN yes | dnf install -y tigervnc-server iproute

RUN cd "${TMOE_DIR}" \
    && cd git/share/old-version/tools/gui \
    && cp startvnc stopvnc startxsdl x11vncpasswd /usr/local/bin \
    && cd /etc/X11/xinit/ \
    && cp Xsession Xsession.bak \
    && echo "dbus-launch startdde" > Xsession \
    && chmod a+rx Xsession

ARG ARCH
COPY --chmod=755 electron/install_apps /tmp
RUN . /tmp/install_apps

WORKDIR /root

# clean
RUN rm -rfv \
    ~/.vnc/*passwd \
    ~/.cache/* \
    /tmp/* \
    2>/dev/null
RUN dnf clean all

EXPOSE 5902 36080
CMD ["bash"]
