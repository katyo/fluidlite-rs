/*!
# Unsafe fluidlite bindings

This crate provides generated unsafe Rust bindings to [fluidlite](https://github.com/katyo/fluidlite) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crate/fluidlite).
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const FLUID_OK: std::os::raw::c_int = 0;
pub const FLUID_FAILED: std::os::raw::c_int = -1;

#[cfg(feature = "generate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "generate-bindings"))]
include!(concat!("bindings/", env!("FLUIDLITE_BINDINGS")));
