/*!
# Unsafe fluidlite bindings

This crate provides generated unsafe Rust bindings to [fluidlite](https://github.com/katyo/fluidlite) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crate/fluidlite).
 */

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![cfg_attr(test, allow(deref_nullptr))]

pub const FLUID_OK: std::os::raw::c_int = 0;
pub const FLUID_FAILED: std::os::raw::c_int = -1;

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(concat!("bindings/", env!("FLUIDLITE_BINDINGS")));

#[cfg(test)]
mod tests {
    use super::*;
    use std::{mem::MaybeUninit, os::raw::c_int};

    #[test]
    fn library_version() {
        let mut major = MaybeUninit::<c_int>::uninit();
        let mut minor = MaybeUninit::<c_int>::uninit();
        let mut micro = MaybeUninit::<c_int>::uninit();

        unsafe {
            fluid_version(major.as_mut_ptr(), minor.as_mut_ptr(), micro.as_mut_ptr());
        }

        let version = unsafe {
            [
                major.assume_init(),
                minor.assume_init(),
                micro.assume_init(),
            ]
        };

        assert!(version[0] == 1);
        assert!(version[1] >= 2);
    }
}
