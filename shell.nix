{pkgs ? import <nixpkgs> {
    overlays = [(import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))];
  }}:
with pkgs;
 mkShell {
    nativeBuildInputs = [
      pkg-config
    ];
    buildInputs = [
      (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
      rust-analyzer
      udev
      alsa-lib
      vulkan-loader
      xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
      libxkbcommon wayland
    ];
  }