{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let
  llvmPackages = llvmPackages_11;
  clang = llvmPackages.clang-unwrapped;
  libclang = llvmPackages.libclang;
  llvm = llvmPackages.llvm;

  stdenv = llvmPackages.stdenv;

in stdenv.mkDerivation {
  name = "fluidlite";

  LIBCLANG_PATH = "${libclang}/lib";

  buildInputs = [
    pkgconfig
    glibc_multi.dev
    clang
    llvm
    libclang
    openssl
    libssh
    libgit2
  ];
}
