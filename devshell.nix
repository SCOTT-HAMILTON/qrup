{ pkgs ? import <nixpkgs> {} }:

let
  moz_overlay = import <rust-overlay>;
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  myrust = nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [ "rust-src" ];
  };
in

with nixpkgs; mkShell {
  buildInputs = [
    cargo
    myrust
    clang
  ];
  shellHook = ''
    
  '';
}

