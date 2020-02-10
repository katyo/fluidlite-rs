/*!
# Rust fluidlite bindings

This project aims provide safe Rust bindings to [fluidlite](https://github.com/katyo/fluidlite) C library.

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

```no_run
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

 */

// Force linking with libfluidlite
#[cfg(test)]
use fluidlite_lib as _;

mod types;
mod private;
mod font;
mod loader;
mod settings;
mod synth;
mod log;
mod version;

pub use self::types::*;
pub use self::font::*;
pub use self::loader::*;
pub use self::settings::*;
pub use self::synth::*;
pub use self::log::*;
pub use self::version::*;

pub(crate) use fluidlite_sys as ffi;
