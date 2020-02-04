# Rust fluidlite bindings

[![License: LGPL-2.1](https://img.shields.io/badge/License-LGPL--2.1-brightgreen.svg)](https://opensource.org/licenses/GPL-2.1)
[![Crates.io Package](https://img.shields.io/crates/v/fluidlite.svg?style=popout)](https://crates.io/crates/fluidlite)
[![Docs.rs API Docs](https://docs.rs/fluidlite/badge.svg)](https://docs.rs/fluidlite)
[![Travis-CI Status](https://travis-ci.com/katyo/fluidlite-rs.svg?branch=master)](https://travis-ci.com/katyo/fluidlite-rs)

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

* __generate-bindings__ Force generate bindings itself instead of use pre-generated
