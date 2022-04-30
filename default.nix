{
  pkgs ?
    import (builtins.fetchGit {
      name = "nixpkgs-with-glibc-2-33";
      url = "https://github.com/NixOS/nixpkgs/";
      ref = "refs/heads/nixpkgs-unstable";
      rev = "c82b46413401efa740a0b994f52e9903a4f6dcd5";
    }) {}
, pkgsWithRust_1_41_0 ?
    import (builtins.fetchGit {
      name = "nixpkgs-with-rustc-1-41-0";
      url = "https://github.com/NixOS/nixpkgs/";
      ref = "refs/heads/nixpkgs-unstable";
      rev = "fcc8660d359d2c582b0b148739a72cec476cfef5";
    }) {}
}:

let
  rustPlatform = pkgs.makeRustPlatform {
    inherit (pkgsWithRust_1_41_0) cargo rustc;
  };
in
pkgs.callPackage ./amethyst-action-game-3d.nix { inherit rustPlatform; }
