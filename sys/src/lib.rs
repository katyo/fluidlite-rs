/*!
# Unsafe fluidlite bindings

[![github](https://img.shields.io/badge/github-katyo/fluidlite--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/fluidlite-rs)
[![crate](https://img.shields.io/crates/v/fluidlite-sys.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/fluidlite-sys)
[![docs](https://img.shields.io/badge/docs.rs-fluidlite--sys-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/fluidlite-sys)
[![LGPL-2.1](https://img.shields.io/badge/License-LGPL--2.1-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/LGPL-2.1)
[![CI](https://img.shields.io/github/workflow/status/katyo/fluidlite-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/fluidlite-rs/actions?query=workflow%3ARust)

This crate provides generated unsafe Rust bindings to [fluidlite](https://github.com/katyo/fluidlite) C library.

Probably this isn't that you really need. See [safe bindings](https://crates.io/crates/fluidlite).
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
