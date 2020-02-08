mod midi;
mod low;
mod sfont;
mod reverb;
mod chorus;
mod params;
mod gen;
mod tuning;
mod misc;
//mod write;

use crate::{ffi, Settings};

/**
The synth object

You create a new synthesizer with `Synth::new()`.
Use the settings structure to specify the synthesizer characteristics.

You have to load a SoundFont in order to hear any sound.
For that you use the `Synth::sfload()` function.

You can use the audio driver functions described below to open
the audio device and create a background audio thread.

The API for sending MIDI events is probably what you expect:
`Synth::noteon()`, `Synth::noteoff()`, ...
 */
pub struct Synth {
    handle: *mut ffi::fluid_synth_t,
    settings: Settings,
}

impl Synth {
    /**
    Creates a new synthesizer object.

    As soon as the synthesizer is created, it will start playing.
     */
    pub fn new(settings: Settings) -> Option<Self> {
        let handle = unsafe { ffi::new_fluid_synth(settings.ptr()) };

        if handle.is_null() {
            return None;
        }

        Self { handle, settings }.into()
    }

    /**
    Set synth sample rate
     */
    pub fn set_sample_rate(&self, sample_rate: f32) {
        unsafe { ffi::fluid_synth_set_sample_rate(self.handle, sample_rate); }
    }

    /**
    Get a reference to the settings of the synthesizer.
     */
    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        unsafe { ffi::delete_fluid_synth(self.handle); }
    }
}
