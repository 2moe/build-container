# syntax=docker/dockerfile:1
FROM cake233/arch-zsh-${TARGETARCH}${TARGETVARIANT}
ARG TAG
ARG OS
RUN pacman -Syu --noconfirm --needed \
    base-devel \
    man-db \
    man-pages

WORKDIR /tmp
COPY --chmod=755 gen_tool /tmp
RUN . gen_tool

ARG AUTO_INSTALL_GUI=true
RUN bash install-gui.sh


RUN pacman -R \
    --noconfirm \
    $(pacman -Qdtq); \
    rm -rfv ~/.vnc/*passwd 2>/dev/null

RUN rm -rfv \
    /var/cache/pacman/pkg/* \
    ~/.cache/* \
    /tmp/*  \
    2>/dev/null; \
    yes | pacman -Scc

EXPOSE 5902 36080
CMD ["/usr/bin/zsh"]
