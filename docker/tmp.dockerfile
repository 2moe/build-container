FROM cake233/arch-arm64

RUN uname -a

RUN curl -Lo paru.tar.zst l.tmoe.me/paru-arm64-github\
    && ls -lah