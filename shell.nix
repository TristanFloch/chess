# https://nixos.wiki/wiki/Rust#Shell.nix_example

{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ rustc cargo gcc rls ];
  buildInputs = with pkgs; [ rustfmt clippy ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
