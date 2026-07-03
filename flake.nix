{
  description = "Vulkan Exposer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          vulkan-loader
          libGL
          libxkbcommon
          libX11
          libXcursor
          libXi
          libXrandr
          libXinerama
          libXrender
          libXfixes
          libXext
          libxcb
        ];

        # Tell dynamic loaders where to find these libraries at runtime
        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath buildInputs}"
        '';
      };
    };
}
