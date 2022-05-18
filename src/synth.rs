mod chorus;
mod count;
mod font;
mod gen;
mod loader;
mod low;
mod midi;
mod misc;
mod params;
mod reverb;
mod tuning;
mod write;

use std::ptr::NonNull;

pub use self::tuning::TuningIter;
pub use self::write::IsSamples;

use crate::{ffi, result_from_ptr, Result, Settings, SettingsRef};

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
#[repr(transparent)]
pub struct Synth {
    handle: NonNull<ffi::fluid_synth_t>,
}

unsafe impl Send for Synth {}

impl Synth {
    /**
    Creates a new synthesizer object.

    As soon as the synthesizer is created, it will start playing.
     */
    pub fn new(settings: Settings) -> Result<Self> {
        result_from_ptr(unsafe { ffi::new_fluid_synth(settings.into_ptr().as_ptr()) })
            .map(|handle| Self { handle })
    }

    /**
    Set synth sample rate
     */
    pub fn set_sample_rate(&self, sample_rate: f32) {
        unsafe {
            ffi::fluid_synth_set_sample_rate(self.handle.as_ptr(), sample_rate);
        }
    }

    /**
    Get a reference to the settings of the synthesizer.
     */
    pub fn get_settings(&self) -> SettingsRef<'_> {
        unsafe {
            let settings = ffi::fluid_synth_get_settings(self.handle.as_ptr());
            SettingsRef::from_ptr(NonNull::new_unchecked(settings))
        }
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        unsafe {
            let _settings = ffi::fluid_synth_get_settings(self.handle.as_ptr());
            let _settings = Settings::from_ptr(NonNull::new_unchecked(_settings));
            ffi::delete_fluid_synth(self.handle.as_ptr());
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Settings, Synth};
    use byte_slice_cast::AsByteSlice;
    use std::{fs::File, io::Write};

    #[test]
    fn synth_sf2() {
        let mut pcm = File::create("Boomwhacker.sf2.pcm").unwrap();

        let settings = Settings::new().unwrap();

        let synth = Synth::new(settings).unwrap();

        synth.sfload("sf_/Boomwhacker.sf2", true).unwrap();

        let mut samples = [0f32; 44100 * 2];

        synth.note_on(0, 60, 127).unwrap();

        synth.write(samples.as_mut()).unwrap();
        pcm.write_all(samples.as_byte_slice()).unwrap();

        synth.note_off(0, 60).unwrap();

        synth.write(samples.as_mut()).unwrap();
        pcm.write_all(samples.as_byte_slice()).unwrap();

        drop(synth);
    }

    #[test]
    fn synth_sf3() {
        let mut pcm = File::create("Boomwhacker.sf3.pcm").unwrap();

        let settings = Settings::new().unwrap();

        let synth = Synth::new(settings).unwrap();

        synth.sfload("sf_/Boomwhacker.sf3", true).unwrap();

        let mut samples = [0f32; 44100 * 2];

        synth.note_on(0, 60, 127).unwrap();

        synth.write(samples.as_mut()).unwrap();
        pcm.write_all(samples.as_byte_slice()).unwrap();

        synth.note_off(0, 60).unwrap();

        synth.write(samples.as_mut()).unwrap();
        pcm.write_all(samples.as_byte_slice()).unwrap();

        drop(synth);
    }
}
