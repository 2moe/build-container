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
RUN . /tmp/set_locale
ENV LANG en_US.UTF-8

WORKDIR /root

# base
RUN pacman -Syyu --needed --noconfirm base base-devel wget curl sudo

RUN groupadd -f \
    --non-unique \
    runner \
    --gid 1002 \
    && useradd -m \
    --gid 1002 \
    --uid 1002 \
    --non-unique \
    runner \
    && echo "runner ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/runner \
    && printf "%s\n" "root:root" | chpasswd \
    && printf "%s\n" "runner:runner" | chpasswd

# clean /var/cache/pacman/
RUN yes | pacman -Scc

CMD ["bash"]
