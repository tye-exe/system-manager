{
  description = "A program to manage tye's nixos configuration.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # For distribution
        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal)
        );
        # For development
        dev = (crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
        );

        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs =
            with pkgs;
            [ installShellFiles ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.libiconv
            ];
        };

        system-manager = craneLib.buildPackage (
          commonArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly commonArgs;

            postInstall =
              let
                name = (craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; }).pname;
              in
              ''
                $out/bin/system-manager completions fish > ${name}.fish
                $out/bin/system-manager completions bash > ${name}.bash
                $out/bin/system-manager completions zsh > ${name}.zsh
                installShellCompletion ${name}.{fish,bash,zsh}
              '';
          }

        );
      in
      {
        checks = {
          inherit system-manager;
        };

        packages.default = system-manager;
        packages.system-manager = system-manager;

        apps.default = flake-utils.lib.mkApp {
          drv = system-manager;
        };

        devShells.default = dev.devShell {
          checks = self.checks.${system};
          packages = [ ];
        };
      }
    );
}
