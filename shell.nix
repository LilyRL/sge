{
  pkgs ? import <nixpkgs> { },
}:
let
  libPath =
    with pkgs;
    lib.makeLibraryPath [
      libGL
      mesa
      vulkan-loader
      vulkan-validation-layers
      libxkbcommon
      wayland
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      libudev-zero
      xorg.libXrandr
      alsa-lib
      libglvnd
      stdenv.cc.cc.lib
    ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    vulkan-tools
    vulkan-headers
    pkg-config
    libGL
    mesa
    libudev-zero
    vulkan-loader
    vulkan-validation-layers
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    alsa-lib
    alsa-utils
    alsa-tools
    alsa-oss
    perf
  ];
  RUST_LOG = "debug";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = libPath;
  VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
  shellHook = ''
    export RUST_LOG=warn
    alias cr="cargo run"
    alias crr="cargo run --release"
    alias ca="cargo add"
    alias cb="cargo build"
    alias ce="cargo run --example"
  '';
}
