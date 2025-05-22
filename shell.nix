{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    pkgs.SDL2
    pkgs.SDL2_gfx
  ];
}
