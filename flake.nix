# flake.nix
{
  description = "C++ GLFW Wayland window";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          gcc
          pkg-config
          glfw
          mesa  # libGL
        ];

        shellHook = ''
          echo "Dev shell ready. Build: g++ main.cpp -o app $(pkg-config --cflags --libs glfw3) -lGL"
          echo "Run: ./app"
        '';
      };
    };
}
