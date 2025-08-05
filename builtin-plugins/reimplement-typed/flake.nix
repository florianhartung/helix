{
  description = "bla";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    # nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";

    utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };
  outputs = { self, nixpkgs, utils, fenix, devshell, rust-overlay, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ devshell.overlays.default (import rust-overlay) ];
        };

        rust-toolchain = pkgs.rust-bin.stable."1.84.0".default.override {
            extensions = [ "rust-src" ];
            targets = [
              "wasm32-unknown-unknown"
              "wasm32-wasip1"
            ];
          };
      in rec{
        # a devshell with all the necessary bells and whistles
        devShells.default = (pkgs.devshell.mkShell {
          name = "bla";
          # packages = [];
          packages = with pkgs; [
            stdenv.cc
            coreutils
            rust-toolchain
            rust-analyzer
            cargo-expand
            cargo-component
          ];
        });
      });
}

