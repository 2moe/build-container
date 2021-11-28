# syntax=docker/dockerfile:1
#---------------------------
FROM busybox
COPY rootfs.tar.zst /root
