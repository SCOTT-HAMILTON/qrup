{ pkgs ? import <nixpkgs> {} }:
let
  rustupToolchain = "nightly";

  rustBuildTargetTriple = "arm-unknown-linux-musleabihf";
  rustBuildHostTriple = "x86_64-unknown-linux-gnu";

  pkgs-cross = import pkgs.path {
    crossSystem = pkgs.lib.systems.examples.raspberryPi;
  };
in

pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    pkgs-cross.stdenv.cc
    yq
    pkgs-cross.musl
  ];
  # Avoid polluting home dir with local project stuff.
  RUSTUP_HOME = toString ./.rustup;
  CARGO_HOME = toString ./.cargo;
  WINEPREFIX = toString ./.wine;

  RUSTUP_TOOLCHAIN = rustupToolchain;

  # Set windows as the default cargo target so that we don't
  # have use the `--target` argument on every `cargo` invocation.
  CARGO_BUILD_TARGET = rustBuildTargetTriple;
  # Set wine as our cargo runner to allow the `run` and `test`
  # command to work.
  CARGO_TARGET_ARM_UNKNOWN_LINUX_MUSLEABIHF_LINKER =
    "${pkgs-cross.stdenv.cc.targetPrefix}cc";

  shellHook = ''
    export PATH=$PATH:${CARGO_HOME}/bin
    export PATH=$PATH:${RUSTUP_HOME}/toolchains/${rustupToolchain}-${rustBuildHostTriple}/bin/

    # Ensures our windows target is added via rustup.
    rustup target add "${rustBuildTargetTriple}"
    '';
  RUSTFLAGS = 
    "-C link-arg=-Wl,-dynamic-linker,/lib/ld-linux-armhf.so.3 " +
    "-C target-feature=+crt-static";
}
