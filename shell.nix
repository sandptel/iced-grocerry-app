# example shell.nix for a Rust project using dlopen libraries
# https://github.com/max-privatevoid/iced/blob/32d007be56a829cdcf2348c023a27dd14b4d36d5/DEPENDENCIES.md#nixos
{ pkgs ? import <nixpkgs> { } }:

let
  dlopenLibraries = with pkgs; [
    libxkbcommon

    # GPU backend
    vulkan-loader
    # libGL

    # Window system
    wayland
    # xorg.libX11
    # xorg.libXcursor
    # xorg.libXi
  ];
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
  ];

  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${pkgs.lib.makeLibraryPath dlopenLibraries}";
}