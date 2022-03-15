# syntax=docker/dockerfile:1
#---------------------------
FROM cake233/arch-${TARGETARCH}${TARGETVARIANT}

# test:
# FROM cake233/arch-amd64 

# ADD code.js /root/readme.js
# COPY --chmod=777 code/init /root

ENV TMOE_CHROOT=true \
    TMOE_DOCKER=true \
    TMOE_DIR="/usr/local/etc/tmoe-linux" \
    DOTNET_RUNNING_IN_CONTAINER=true \
    DOTNET_ROOT="/usr/local/dotnet" \
    # ${HOME}/.dotnet/tools:$PATH
    PATH="/usr/local/powershell:/usr/local/dotnet/bin:$PATH" \
    # Unset ASPNETCORE_URLS from aspnet base image
    ASPNETCORE_URLS="" \
    # Do not generate certificate
    DOTNET_GENERATE_ASPNET_CERTIFICATE=false \
    # Enable correct mode for dotnet watch (only mode supported in a container)
    DOTNET_USE_POLLING_FILE_WATCHER=true \
    # Unset Logging__Console__FormatterName from aspnet base image
    Logging__Console__FormatterName="" \
    # Skip extraction of XML docs - generally not useful within an image/container - helps performance
    NUGET_XMLDOC_MODE=skip 

# install base-devel
RUN pacman -Syu --noconfirm --needed base base-devel
# install dependencies
RUN pacman \
    -S \
    --noconfirm \
    --needed \
    git \
    unzip \
    neofetch \
    iproute

# git clone tmoe
ARG URL="https://github.com/2moe/tmoe-linux"
RUN mkdir -p "$TMOE_DIR"/environment \
    && cd "$TMOE_DIR" \
    && git clone \
    -b master \
    --depth=1 \
    "$URL" git

# export env to file
RUN cd "$TMOE_DIR"; \
    printf "%s\n" \
    'export PATH="/usr/local/powershell:/usr/local/dotnet/bin${PATH:+:${PATH}}"' \
    'export DOTNET_ROOT="/usr/local/dotnet"' \
    'export DOTNET_RUNNING_IN_CONTAINER=true' \
    'export ASPNETCORE_URLS=""' \
    'export DOTNET_GENERATE_ASPNET_CERTIFICATE=false' \
    'export DOTNET_USE_POLLING_FILE_WATCHER=true' \
    'export Logging__Console__FormatterName=""' \
    'export NUGET_XMLDOC_MODE="skip"' \ 
    > environment/container.env; \
    chmod -R a+rx environment/

# install dotnet
# https://docs.microsoft.com/dotnet/core/install/linux-scripted-manual#scripted-install
RUN cd /tmp \
    && mkdir -p /usr/local/dotnet \
    && curl -LO https://dot.net/v1/dotnet-install.sh \
    && yes | bash dotnet-install.sh -c Current --install-dir /usr/local/dotnet \
    && cd /usr/local/dotnet \
    && mkdir -p bin \
    && ln -svf ../dotnet ./bin/ \
    && chmod a+rx -v bin

# install pwsh
RUN mkdir -p /usr/local/powershell \
    && dotnet tool install --tool-path /usr/local/powershell PowerShell \
    && chmod a+rx -Rv /usr/local/powershell

ARG OS
ARG TAG
ARG ARCH
COPY --chmod=755 set_container_txt /tmp
RUN . /tmp/set_container_txt

# RUN cd "$TMOE_DIR"; \
#     printf "%s\n" \
#     'cd ~' \
#     "/usr/local/powershell/pwsh" \
#     > environment/entrypoint; \
#     chmod -v a+rx environment/*

# Do not show first run text
ARG DOTNET_NOLOGO=true
# export version info to file
RUN cd /root; \
    dotnet --version; \
    printf "%s\n" \
    "" \
    '[version]' \
    "ldd = '$(ldd --version 2>&1 | head -n 2 | grep -vi copyright | sed ":a;N;s/\n/ /g;ta")'" \
    "git = '$(git --version)'" \
    "dotnet = '$(dotnet --version)'" \
    "powershell = '$(pwsh -Version)'" \
    "dotnet_info = '''" \
    "$(dotnet --info)" \
    "'''" \
    "" \
    '[other]' \
    'cmd = "/usr/local/powershell/pwsh"' \
    > version.toml

# add archlinux mirror repo & install fakeroot-tcp
RUN cd /tmp; \
    cp -fv "${TMOE_DIR}"/git/share/old-version/tools/sources/yay/build_fakeroot ./; \
    chmod a+rx -v build_fakeroot; \
    ./build_fakeroot --add-arch_for_edu-repo; \
    ./build_fakeroot --add-archlinuxcn-repo; \
    ./build_fakeroot --install-paru; \
    ./build_fakeroot --install-fakeroot; \
    ./build_fakeroot --archlinux-repo-mirror; \
    cat /etc/pacman.conf

# clean
RUN rm -rfv \
    /var/cache/pacman/pkg/* \
    ~/.cache/* \
    /tmp/*  \
    2>/dev/null; \
    yes | pacman -Scc

CMD [ "/usr/local/powershell/pwsh" ]
