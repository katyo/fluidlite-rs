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

* __generate-bindings__ Force generate bindings itself instead of use pre-generated

## Example

```ignore
use std::fs::File;
use fluidlite::{};

fn main() {
    let settings = Settings::default();

    let synth = Synth::new(settings);
    synth.sfload("soundfont.sf3", 1);

    let buffer = new float[44100*2];

    let file = File::create("float32output.pcm");

    synth.noteon(0, 60, 127);
    synth.write_float(44100, buffer, 0, 2, buffer, 1, 2);
    file.write(buffer);

    synth.noteoff(0, 60);
    synth.write_float(44100, buffer, 0, 2, buffer, 1, 2);
    file.write(buffer);
}
```

 */

// Force linking with libfluidlite
#[cfg(test)]
use fluidlite_lib as _;

mod settings;
mod synth;
mod log;
mod version;

pub use self::settings::*;
pub use self::synth::*;
pub use self::log::*;
pub use self::version::*;

pub(crate) use fluidlite_sys as ffi;

use std::result::{Result as StdResult};

pub type Result<T> = StdResult<T, String>;
pub type Status = Result<()>;
