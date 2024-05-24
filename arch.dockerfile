FROM library/archlinux:latest

RUN pacman -Syu kde-development-environment-meta sudo base-devel --noconfirm
RUN useradd -m -d /build ci
RUN pacman -S kconfig kcoreaddons gcc-libs glib2 glibc kconfig kcoreaddons kcrash kdbusaddons ki18n kirigami kirigami-addons kwidgetsaddons qt6 --noconfirm

USER ci
WORKDIR /build

ADD --chown=ci:ci . /build/

RUN cmake . -B build
RUN cmake --build build
RUN cp build/bin/jewels jewels
RUN makepkg