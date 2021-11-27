# syntax=docker/dockerfile:1
FROM cake233/debian-zsh-${TARGETARCH}${TARGETVARIANT}

ARG DEBIAN_FRONTEND=noninteractive

ARG TAG
ARG OS

WORKDIR /tmp
COPY --chmod=755 gen_tool /tmp
RUN . gen_tool

ARG AUTO_INSTALL_GUI=true
RUN bash install-gui.sh

WORKDIR /root
RUN rm -fv /var/cache/apt/archives/* \
    /var/cache/apt/* \
    /var/mail/* \
    /var/log/* \
    /var/log/apt/* \
    /var/log/journal/* \
    /var/lib/apt/lists/* \
    2>/dev/null

EXPOSE 5902 36080
CMD ["zsh"]
