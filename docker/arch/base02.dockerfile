# syntax=docker/dockerfile:1
#---------------------------
FROM scratch
ADD arch.tar /

ARG TARGETARCH
COPY --chmod=755 arch_key /tmp
# SHELL [ "bash", "-cex" ]
RUN . /tmp/arch_key

# set locale
COPY --chmod=755 set_locale /tmp
RUN . set_locale
ENV LANG en_US.UTF-8

WORKDIR /root

# base
RUN pacman -Syyu --needed --noconfirm base wget curl

# clean /var/cache/pacman/
RUN yes | pacman -Scc

CMD ["/usr/bin/bash"]
