{
  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };
        rust = pkgs.latest.rustChannels.nightly.rust;
        rust-src = rust.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" "x86_64-unknown-linux-gnu" ];
        };
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo-asm
            cargo-flamegraph
            gnuplot
            nodejs
            rust-src
            wasm-pack
          ];
        };
      });
}
