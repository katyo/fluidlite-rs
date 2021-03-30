# Rust fluidlite bindings

[![github](https://img.shields.io/badge/github-katyo/fluidlite--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/fluidlite-rs)
[![crate](https://img.shields.io/crates/v/fluidlite.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/fluidlite)
[![docs](https://img.shields.io/badge/docs.rs-fluidlite-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/fluidlite)
[![LGPL-2.1](https://img.shields.io/badge/License-LGPL--2.1-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/LGPL-2.1)
[![CI](https://img.shields.io/github/workflow/status/katyo/fluidlite-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/fluidlite-rs/actions?query=workflow%3ARust)

This project aims provide safe Rust bindings to [fluidlite](https://github.com/divideconcept/FluidLite) C library.

> FluidLite is a very light version of FluidSynth designed to be hardware,
> platform and external dependency independant. It only uses standard C libraries.
>
> It also adds support for SF3 files (SF2 files compressed with ogg vorbis)
> and an additional setting to remove the constraint of channel 9 (drums):
> fluid_settings_setstr(settings, "synth.drums-channel.active", "no");
> you can still select bank 128 on any channel to use drum kits.
>
> FluidLite keeps very minimal functionnalities (settings and synth),
> therefore MIDI file reading, realtime MIDI events and audio output
> must be implemented externally.

## Crates

* [__fluidlite__](https://crates.io/crates/fluidlite) Safe bindings
* [__fluidlite-sys__](https://crates.io/crates/fluidlite-sys) Unsafe bindings (generated using bindgen)
* [__fluidlite-lib__](https://crates.io/crates/fluidlite-lib) Bundled library

## Features

* __generate-bindings__ Force generate bindings on build instead of using pre-generated

## Example

```rust
use std::{
    fs::File,
    io::Write,
};
use byte_slice_cast::AsByteSlice;
use fluidlite::{Settings, Synth};

let settings = Settings::new().unwrap();

let synth = Synth::new(settings).unwrap();
synth.sfload("soundfont.sf3", true).unwrap();

let mut buffer = [0i16; 44100 * 2];

let mut file = File::create("soundfont-sample.pcm").unwrap();

synth.note_on(0, 60, 127).unwrap();
synth.write(buffer.as_mut()).unwrap();
file.write(buffer.as_byte_slice()).unwrap();

synth.note_off(0, 60).unwrap();
synth.write(buffer.as_mut()).unwrap();
file.write(buffer.as_byte_slice()).unwrap();
```
