use crate::{ffi, Synth};

impl Synth {
    /**
    Returns the number of MIDI channels that the synthesizer uses internally
     */
    pub fn count_midi_channels(&self) -> u32 {
        unsafe { ffi::fluid_synth_count_midi_channels(self.handle) as _ }
    }

    /**
    Returns the number of audio channels that the synthesizer uses internally
     */
    pub fn count_audio_channels(&self) -> u32 {
        unsafe { ffi::fluid_synth_count_audio_channels(self.handle) as _ }
    }

    /**
    Returns the number of audio groups that the synthesizer uses internally.
    This is usually identical to audio_channels.
     */
    pub fn count_audio_groups(&self) -> u32 {
        unsafe { ffi::fluid_synth_count_audio_groups(self.handle) as _ }
    }

    /**
    Returns the number of effects channels that the synthesizer uses internally
     */
    pub fn count_effects_channels(&self) -> u32 {
        unsafe { ffi::fluid_synth_count_effects_channels(self.handle) as _ }
    }
}
