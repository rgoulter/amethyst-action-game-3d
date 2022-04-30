{
  pkgs ?
    import (builtins.fetchGit {
      name = "nixpkgs-with-rustc-1-41-0";
      url = "https://github.com/NixOS/nixpkgs/";
      ref = "refs/heads/nixpkgs-unstable";
      rev = "fcc8660d359d2c582b0b148739a72cec476cfef5";
    }) {}
}:

with pkgs;
mkShell {
  buildInputs = [
    alsaLib
    cmake
    cargo
    freetype
    expat
    openssl
    pkgconfig
    python3
    rustc
    vulkan-validation-layers
    xlibs.libX11
  ];

  APPEND_LIBRARY_PATH = lib.makeLibraryPath [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
  '';
}
