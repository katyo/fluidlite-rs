/*!
# Bundled fluidlite library

This crate provides bundled [fluidlite](https://github.com/divideconcept/FluidLite) C library
for using with [__fluidlite__](https://crates.io/crates/fluidlite) crate in case
when system-wide library is not available.

## Usage

You can simply add this as dependency to your manifest:

```toml
[dependencies]
fluidlite = "^0.1"

# Use bundled library to avoid unresolved links
fluidlite-lib = "^0.1"
```

## Features

You can apply some customizations to library using those features:

- __shared__ Force bundle shared (or dynamic) library instead of static
- __with-sf3__ Enable SoundFont3 support which requires ogg/vorbis (system-wide or bundled)

 */

#[cfg(test)]
mod tests {
    use std::{
        mem::MaybeUninit,
        os::raw::c_int,
    };

    extern "C" {
        pub fn fluid_version(
            major: *mut c_int,
            minor: *mut c_int,
            micro: *mut c_int,
        );
    }

    #[test]
    fn library_version() {
        let mut major = MaybeUninit::<i32>::uninit();
        let mut minor = MaybeUninit::<i32>::uninit();
        let mut micro = MaybeUninit::<i32>::uninit();

        unsafe { fluid_version(major.as_mut_ptr(), minor.as_mut_ptr(), micro.as_mut_ptr()); }

        let version = unsafe { [major.assume_init(), minor.assume_init(), micro.assume_init()] };

        assert_eq!(&version, &[1, 1, 0]);
    }
}
