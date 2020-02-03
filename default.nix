{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let
  llvmPackages = llvmPackages_latest;
  clang = llvmPackages.clang-polly-unwrapped;
  libclang = llvmPackages.libclang;
  llvm = llvmPackages.llvm-polly;

  stdenv = llvmPackages.stdenv;

in stdenv.mkDerivation {
  name = "fluidlite";

  LIBCLANG_PATH = "${libclang}/lib";

  buildInputs = [
    glibc_multi.dev
    clang
    llvm
    libclang
  ];
}
