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

## Features

 * __bindgen__ Force generate bindings itself instead of use pre-generated
 * __builtin__ Force compile builtin _fluidlite_ C-library
 * __pkg-config__ Use _pkg-config_ to find installed libraries
 * __with-sf3__ Enable _SoundFont3_ support (SF2 with vorbis-encoded samples)
 * __with-stb__ Use _stb-vorbis_ decoder instead of _libvorbis_/_libogg_.
 * __shared__ Build shared _fluidlite_ C-library
 * __static__ Build static _fluidlite_ C-library

When __pkg-config__ feature is used the installed __fluidlite__ library will be used if found. To force build and link builtin version you can use __builtin__ feature.

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

mod font;
mod loader;
mod log;
mod private;
mod settings;
mod synth;
mod types;
mod version;

pub use self::font::*;
pub use self::loader::*;
pub use self::log::*;
pub use self::settings::*;
pub use self::synth::*;
pub use self::types::*;
pub use self::version::*;

pub(crate) use fluidlite_sys as ffi;
