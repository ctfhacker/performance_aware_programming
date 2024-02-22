let

  orig_pkgs = import (fetchTarball("channel:nixpkgs-23.11-darwin")) {};

  rust-overlay = import(orig_pkgs.fetchFromGitHub {
    owner = "oxalica";
    repo = "rust-overlay";
    rev = "7a94fe7690d2bdfe1aab475382a505e14dc114a6";
    sha256 = "sha256-/DZsoPH5GBzOpVEGz5PgJ7vh8Q6TcrJq5u8FcBjqAfI=";
  });

  pkgs = orig_pkgs.extend rust-overlay;

in pkgs.mkShell {
  buildInputs = [ 
    pkgs.rust-bin.nightly.latest.default
    pkgs.rust-analyzer
    pkgs.pkg-config
  ];
}
