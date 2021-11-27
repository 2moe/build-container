# syntax=docker/dockerfile:1

FROM scratch
ADD arch.tar /

RUN pacman-key --init
ARG TARGETARCH
COPY --chmod=755 arch_key /tmp

SHELL [ "bash", "-cex" ]
WORKDIR /tmp
RUN . ./arch_key

# set locale
COPY --chmod=755 set_locale /tmp
RUN . set_locale
ENV LANG en_US.UTF-8

WORKDIR /root

# base
RUN pacman -Syyu --needed --noconfirm base wget

# clean /var/cache/pacman/
RUN yes | pacman -Scc

CMD ["bash"]
