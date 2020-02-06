# Bundled fluidlite library

[![License: LGPL-2.1](https://img.shields.io/badge/License-LGPL--2.1-brightgreen.svg)](https://opensource.org/licenses/LGPL-2.1)
[![Crates.io Package](https://img.shields.io/crates/v/fluidlite-lib.svg?style=popout)](https://crates.io/crates/fluidlite-lib)
[![Docs.rs API Docs](https://docs.rs/fluidlite-lib/badge.svg)](https://docs.rs/fluidlite-lib)
[![Travis-CI Status](https://travis-ci.com/katyo/fluidlite-rs.svg?branch=master)](https://travis-ci.com/katyo/fluidlite-rs)

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

Next you should say compiler that you want to use that crate:

```rust
// Either in traditional manner
extern crate fluidlite_lib;

// Or in Rust2018 manner
use fluidlite_lib as _;
```

## Features

You can apply some customizations to library using those features:

- __shared__ Force bundle shared (or dynamic) library instead of static
- __with-sf3__ Enable SoundFont3 support which requires ogg/vorbis (system-wide or bundled)
- __with-stb__ Use __stb-vorbis__ instead of Xiph's libogg/libvorbis
