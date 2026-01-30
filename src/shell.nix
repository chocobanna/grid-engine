{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    # toolchain
    gcc
    gdb
    pkg-config

    # SDL3
    sdl3
  ];

  shellHook = ''
    echo "SDL3 dev shell ready."
    echo "Build: cc main.c -o app \$(pkg-config --cflags --libs sdl3)"
    echo "Run (Wayland): SDL_VIDEODRIVER=wayland ./app"
  '';
}
