use crate::{ffi, Loader, Synth};

impl Synth {
    /**
    Add a SoundFont loader to the synthesizer. Note that SoundFont
    loader don't necessarily load SoundFonts. They can load any type
    of wavetable data but export a SoundFont interface.
     */
    pub fn add_sfloader(&self, loader: Loader) {
        unsafe {
            ffi::fluid_synth_add_sfloader(self.handle, loader.into_ptr());
        }
    }
}
