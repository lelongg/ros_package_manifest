let
  mozilla = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ mozilla ]; };
in with pkgs;
let
  rust = (rustChannelOf {
    channel = "1.47.0";
    sha256 = "sha256:1hkisci4as93hx8ybf13bmxkj9jsvd4a9ilvjmw6n64w4jkc1nk9";
  }).rust.override { extensions = [ "rust-src" ]; };
in mkShell {
  buildInputs = [ clang rust openssl pkgconfig ];
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
}
