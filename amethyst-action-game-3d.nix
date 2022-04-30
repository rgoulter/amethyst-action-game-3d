{ lib
, fetchFromGitHub
, rustPlatform
, alsaLib
, cmake
, expat
, freetype
, openssl
, pkg-config
, python3
, rustc
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
    alsaLib.dev
    cmake
    expat
    freetype
    openssl
    rustc
    vulkan-validation-layers
    xorg.libX11.dev
  ];

  # The amethyst crate wants to create a directory under
  # HOME at build time. Set HOME to a tempdir as a workaround.
  preBuild = ''
    export HOME=$(mktemp -d)
  '';

  meta = with lib; {
    description = "A toy project playing around with Rust and the Amethyst library.";
    homepage = "github.com/rgoulter/amethyst-action-game-3d";
    license = licenses.mit;
    maintainers = [ ];
  };
}
