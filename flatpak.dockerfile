FROM library/ubuntu:latest

WORKDIR /build

ADD . /build/

RUN apt update
RUN apt upgrade -y
RUN apt install -y flatpak flatpak-builder
RUN flatpak --system remote-add flathub https://dl.flathub.org/repo/flathub.flatpakrepo
RUN flatpak --system install -y org.kde.Platform//6.7
RUN flatpak --system install -y org.kde.Sdk//6.7
RUN flatpak --system install -y io.qt.qtwebengine.BaseApp//6.7
RUN flatpak-builder build dev.imanuel.jewels.json