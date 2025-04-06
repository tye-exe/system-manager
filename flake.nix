{
  description = "A program to manage tye's nixos configuration.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rust-build = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

      in
      {
        # Set up the development environment.
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              rust-build
              bacon
            ];
          };

        # Build the package.
        packages = rec {
          system-manager = pkgs.rustPlatform.buildRustPackage {
            pname = "system-manager";
            version = "1.2.0";

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
          default = system-manager;
        };
      }
    );
}
