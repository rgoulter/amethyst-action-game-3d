{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain =
          fenix.packages.${system}.toolchainOf {
            channel = "1.41.1";
            sha256 = "sha256-CtlU5P+v0ZJDzYlP4ZLA9Kg5kMEbSHbeYugnhCx0q0Q=";
          };
      in let
        rustPlatform =
          pkgs.makeRustPlatform {
            inherit (toolchain) cargo rustc;
          };
      in
      rec {
        defaultPackage = pkgs.callPackage ./amethyst-action-game-3d.nix { inherit rustPlatform; };
        devShell = import ./shell.nix { inherit pkgs toolchain; };
        apps = {
          action-game-3d = {
            type = "app";
            program = "${defaultPackage}/bin/action-game-3d";
          };
          simple-level = {
            type = "app";
            program = "${defaultPackage}/bin/simple-level";
          };
        };
        defaultApp = apps.action-game-3d;
      });
}
