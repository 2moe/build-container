#!/usr/bin/env bash
#####################
CUR=${PWD}
unset DISRO_CODE
DEBIAN_CHROOT="ubuntu"
ARCH_TYPE=amd64
UBUNTU_URL="http://azure.archive.ubuntu.com/ubuntu"
UBUNTU_URL_02="https://mirrors.bfsu.edu.cn/ubuntu"
DISRO_CODE=$(curl -L ${UBUNTU_URL}/dists/devel/Release | grep 'Codename:' | head -n 1 | awk -F ': ' '{print $2}')
[[ -n ${DISRO_CODE} ]] || DISRO_CODE=$(curl -L ${UBUNTU_URL_02}/dists/devel/Release | grep 'Codename:' | head -n 1 | awk -F ': ' '{print $2}')
echo ${DISRO_CODE}
###################
sudo apt update
sudo apt install -y debootstrap
#qemu-user-static
##################
cd /usr/share/debootstrap/scripts
if [[ ! -e ${DISTRO_CODE} ]]; then
    sudo ln -svf gutsy ${DISTRO_CODE}
fi
cd -
sudo debootstrap --no-check-gpg --arch ${ARCH_TYPE} --components=main,restricted,universe,multiverse --variant=minbase --include=init,locales,ca-certificates,openssl,curl ${DISTRO_CODE} ${DEBIAN_CHROOT} ${UBUNTU_URL} || sudo debootstrap --no-check-gpg --arch ${ARCH_TYPE} --components=main,restricted,universe,multiverse --variant=minbase --include=init,locales,ca-certificates,openssl,curl ${DISTRO_CODE} ${DEBIAN_CHROOT} ${UBUNTU_URL_02}

sudo mkdir -pv ${DEBIAN_CHROOT}/run/systemd
sudo su -c "echo 'docker' >${DEBIAN_CHROOT}/run/systemd/container"

sed -i "s@hirsute@${DISRO_CODE}@g" ${ARCH_TYPE}.list
sudo cp -fv ${ARCH_TYPE}.list ${DEBIAN_CHROOT}/etc/apt/sources.list

cd ${DEBIAN_CHROOT}
tar -cvf ${CUR}/ubuntu.tar ./*
cd ${CUR}
