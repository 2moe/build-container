FROM cake233/arch-arm64

RUN uname -a

RUN curl -Lo paru.tar.zst l.tmoe.me/paru-arm64-github; \
    cat paru.tar.zst; \
    curl -Lo https://l.tmoe.me/paru-arm64-github; \
    tar -xvf paru.tar.zst \
    && ls -lah