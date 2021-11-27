# syntax=docker/dockerfile:1
FROM cake233/fedora-zsh-${TARGETARCH}${TARGETVARIANT}

ARG TAG
ARG OS

WORKDIR /tmp
COPY --chmod=755 gen_tool /tmp
RUN . gen_tool

ARG AUTO_INSTALL_GUI=true
RUN bash install-gui.sh

WORKDIR /root
RUN rm -rfv /tmp/* ~/.vnc/*passwd ~/.cache/* 2>/dev/null; \
    dnf clean all

EXPOSE 5902 36080
CMD ["zsh"]
