{
  description = "CHANGEME";

  # Flake inputs
  inputs = {
    # Nixpkgs
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Rust overlay for Rust nightly
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    # Flake utils for eachDefaultSystem
    flake-utils.url = "github:numtide/flake-utils";
  };

  # Flake outputs
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem(system: 
      let
        # Use the rust overlay to use Rust nightly
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        myrust = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [];
        };
      in
        with pkgs; {
          devShells.default = mkShell {
              packages = [
                myrust
                rust-analyzer
              ];
            };
          });
}
