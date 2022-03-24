FROM alpine

RUN uname -a \
    && apk add curl zstd tar

RUN curl -Lvo paru.tar.zst l.tmoe.me/paru-arm64-github; \
    cat paru.tar.zst; \
    curl -Lvo paru.tar.zst https://l.tmoe.me/paru-arm64-github; \
    tar -xvf paru.tar.zst \
    && ls -lah

