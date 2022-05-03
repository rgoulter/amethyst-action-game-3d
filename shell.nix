{
  pkgs ? import <nixpkgs> {},
  toolchain ?
    let
      fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") { };
    in
    fenix.toolchainOf {
      channel = "1.41.1";
      sha256 = "sha256-CtlU5P+v0ZJDzYlP4ZLA9Kg5kMEbSHbeYugnhCx0q0Q=";
    }
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    alsaLib
    cmake
    freetype
    expat
    openssl
    pkgconfig
    python3
    rust-analyzer
    vulkan-validation-layers
    xorg.libX11
  ] ++ (with toolchain; [
    cargo
    rustc
  ]);

  APPEND_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
    vulkan-loader
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ]);

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
