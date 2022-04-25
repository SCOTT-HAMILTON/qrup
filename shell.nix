let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  pkgs = import <nixpkgs> {};

  # Rolling updates, not deterministic.
  # pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in with pkgs; mkShell {
  buildInputs = [ cargo rustc openssl pkg-config ];
}
