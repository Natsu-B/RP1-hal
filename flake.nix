{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          targets = [ "thumbv7m-none-eabi" ];
          extensions = [ "rust-src" "llvm-tools-preview" ];
        };
        cargoNightlyCompat = pkgs.writeShellScriptBin "cargo" ''
          if [ "''${1:-}" = "+nightly" ]; then
            shift
          fi
          exec ${rustToolchain}/bin/cargo "$@"
        '';
        buildRp1Hal = pkgs.writeShellScriptBin "build-rp1-hal" ''
          if [ -f Cargo.toml ]; then
            exec cargo build --target thumbv7m-none-eabi "$@"
          fi

          echo "RP1-hal has no Cargo workspace yet."
          echo "Create Cargo.toml, then run: cargo build --target thumbv7m-none-eabi"
          exit 1
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            cargoNightlyCompat
            buildRp1Hal
            rustToolchain
            pkgs.binutils
            pkgs.xxd
            pkgs.dtc
            pkgs.cargo-binutils
          ];
        };
      }
    );
}
