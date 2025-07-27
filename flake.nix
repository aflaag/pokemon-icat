{
  description = "pokemon-icat";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        packages = pkgs.callPackage ./default.nix {};
      in {
        packages.default = packages.pokemon-icat;

        # Optional: add devShell
        # devShell = pkgs.mkShell {
        #   buildInputs = with pkgs; [
        #     rustc
        #     cargo
        #     pkg-config
        #     python3
        #     python3Packages.pillow
        #     # any other dev deps you want
        #   ];
        # };
      }
    );
}


# {
#   description = "your description here";
#
#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable"; # or whatever you use
#     flake-utils.url = "github:numtide/flake-utils";
#   };
#
# outputs = { self, nixpkgs, flake-utils }:
#   flake-utils.lib.eachDefaultSystem (system:
#     let
#       pkgs = import nixpkgs { inherit system; };
#
#       src = pkgs.lib.cleanSourceWith {
#         src = ./.;
#         filter = path: type:
#           let baseName = baseNameOf path;
#           in baseName != ".git"
#              && baseName != "target"
#              && baseName != "result"
#              && baseName != ".direnv";
#       };
#
#       packages = pkgs.callPackage ./default.nix {
#         inherit src;
#       };
#     in {
#       packages.default = packages.pokemon-icat;
#       # packages.pokemon-icons = packages.pokemon-icons;
#
#       # devShell = pkgs.mkShell {
#       #   buildInputs = with pkgs; [
#       #     rustc
#       #     cargo
#       #     pkg-config
#       #     python3
#       #     python3Packages.pillow
#       #   ];
#       # };
#     }
#   );
#
# }


