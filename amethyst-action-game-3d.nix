{ lib
, rustPlatform
, alsaLib
, bash
, cmake
, expat
, freetype
, openssl
, pkg-config
, python3
, vulkan-loader
, vulkan-validation-layers
, xorg
}:

rustPlatform.buildRustPackage rec {
  pname = "amethyst-action-game-3d";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = "sha256-PpUi7uHAglJYG/7WkOwskKxUBQzGz6h2fbgF4q/71gY=";

  XDG_DATA_DIRS="$XDG_DATA_DIRS:${vulkan-validation-layers}";

  # For some reason, the files which the amethyst needs at build time
  # don't get included with `cargo vendor`.
  # Workaround this by writing a dummy value to this file.
  depsExtraArgs = {
    postBuild = ''
      echo > amethyst-action-game-3d-0.1.0-vendor.tar.gz/amethyst/.sentry_dsn.txt <<TXT
      https://00000000000000000000000000000000@sentry.io/0000000
      TXT
    '';
  };

  nativeBuildInputs = [
    pkg-config
    python3
  ];

  buildInputs = [
    cmake
    openssl

    alsaLib
    expat
    freetype

    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.libxcb

    vulkan-loader
    vulkan-validation-layers
  ];

  # The amethyst crate wants to create a directory under
  # HOME at build time. Set HOME to a tempdir as a workaround.
  preBuild = ''
    export HOME=$(mktemp -d)
  '';

  postInstall = ''
    mkdir $out/libexec
    mv $out/bin/action-game-3d $out/libexec/action-game-3d
    mv $out/bin/simple-level $out/libexec/simple-level
    find assets -type f -exec install -Dm 555 "{}" "$out/libexec/{}" \;
    find resources -type f -exec install -Dm 555 "{}" "$out/libexec/{}" \;

    cat > $out/bin/action-game-3d <<SH
    #!${bash}/bin/bash
    export LD_LIBRARY_PATH="${vulkan-loader}/lib:$${LD_LIBRARY_PATH}"
    $out/libexec/action-game-3d
    SH
    chmod +x $out/bin/action-game-3d

    cat > $out/bin/simple-level <<SH
    #!${bash}/bin/bash
    export LD_LIBRARY_PATH="${vulkan-loader}/lib:$${LD_LIBRARY_PATH}"
    $out/libexec/simple-level
    SH
    chmod +x $out/bin/simple-level
  '';

  meta = with lib; {
    description = "A toy project playing around with Rust and the Amethyst library.";
    homepage = "github.com/rgoulter/amethyst-action-game-3d";
    license = licenses.mit;
    maintainers = [ ];
  };
}
