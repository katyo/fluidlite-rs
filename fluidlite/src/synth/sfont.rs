use std::{
    ffi::CString,
    path::Path,
};
use crate::{Synth, Result, Status, ffi};

/**
SoundFont management
 */
impl Synth {
    /**
    Loads a SoundFont file and creates a new SoundFont. The newly
    loaded SoundFont will be put on top of the SoundFont
    stack. Presets are searched starting from the SoundFont on the
    top of the stack, working the way down the stack until a preset
    is found.
    \param synth The synthesizer object
    \param filename The file name
    \param reset_presets If non-zero, the presets on the channels will be reset
    \returns The ID of the loaded SoundFont, or -1 in case of error
     */
    pub fn sfload<P: AsRef<Path>>(&self, filename: P, reset_presets: bool) -> Result<u32> {
        let filename = filename.as_ref().to_str().ok_or_else(|| String::from("Invalid UTF-8"))?;
        let filename = CString::new(filename).map_err(|error| error.to_string())?;
        let reset_presets = if reset_presets { 1 } else { 0 };

        self.neg_err(unsafe { ffi::fluid_synth_sfload(self.handle, filename.as_ptr(), reset_presets) })
            .map(|id| id as _)
    }

    /**
    Reload a SoundFont. The reloaded SoundFont retains its ID and
    index on the stack.
    \param synth The synthesizer object
    \param id The id of the SoundFont
    \returns The ID of the loaded SoundFont, or -1 in case of error
     */
    pub fn sfreload(&self, id: u32) -> Result<i32> {
        self.neg_err(unsafe { ffi::fluid_synth_sfreload(self.handle, id) })
    }

    /**
    Removes a SoundFont from the stack and deallocates it.
    \param synth The synthesizer object
    \param id The id of the SoundFont
    \param reset_presets If TRUE then presets will be reset for all channels
    \returns 0 if no error, -1 otherwise
     */
    pub fn sfunload(&self, id: u32, reset_presets: bool) -> Status {
        let reset_presets = if reset_presets { 1 } else { 0 };
        self.zero_ok(unsafe { ffi::fluid_synth_sfunload(self.handle, id, reset_presets) })
    }

    /*
    /**
    Add a SoundFont. The SoundFont will be put on top of
    the SoundFont stack.
    \param synth The synthesizer object
    \param sfont The SoundFont
    \returns The ID of the loaded SoundFont, or -1 in case of error
     */
    pub fn add_sfont(&self, sfont: &SFont) -> Result<i32> {
        self.neg_err(unsafe { ffi::fluid_synth_add_sfont(self.handle, sfont.as_ptr()) })
    }

    /**
    Remove a SoundFont that was previously added using
    fluid_synth_add_sfont(). The synthesizer does not delete the
    SoundFont; this is responsability of the caller.
    \param synth The synthesizer object
    \param sfont The SoundFont
     */
    pub fn remove_sfont(&self, sfont: &SFont) {
        unsafe { ffi::fluid_synth_remove_sfont(self.handle, sfont.as_ptr()); }
    }

    /**
    Count the number of loaded SoundFonts.
    \param synth The synthesizer object
    \returns The number of loaded SoundFonts
     */
    pub fn sfcount(&self) -> Result<usize> {
        self.neg_err(unsafe { ffi::fluid_synth_sfcount(self.handle) })
    }

    /**
    Get a SoundFont. The SoundFont is specified by its index on the
    stack. The top of the stack has index zero.

    \param synth The synthesizer object
    \param num The number of the SoundFont (0 <= num < sfcount)
    \returns A pointer to the SoundFont
     */
    pub fn get_sfont(&self, num: usize) -> Result<&SFont> {
      //FLUIDSYNTH_API fluid_sfont_t* fluid_synth_get_sfont(fluid_synth_t* synth, unsigned int num);
    }

    /**
    Get a SoundFont. The SoundFont is specified by its ID.
    \param synth The synthesizer object
    \param id The id of the sfont
    \returns A pointer to the SoundFont
     */
    pub fn get_sfont_by_id(&self, id: u32) -> Result<SFont> {
        //FLUIDSYNTH_API fluid_sfont_t* fluid_synth_get_sfont_by_id(fluid_synth_t* synth, unsigned int id);
    }

    /**
    Get the preset of a channel
     */
    pub fn get_channel_preset(&self, chan: i32) -> Result<Preset> {
        //FLUIDSYNTH_API fluid_preset_t* fluid_synth_get_channel_preset(fluid_synth_t* synth, int chan);
    }
     */

    /**
    Offset the bank numbers in a SoundFont.
    Returns -1 if an error occured (out of memory or negative offset)
     */
    pub fn set_bank_offset(&self, sfont_id: i32, offset: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_set_bank_offset(self.handle, sfont_id, offset as i32) })
    }

    /**
    Get the offset of the bank numbers in a SoundFont.
     */
    pub fn get_bank_offset(&self, sfont_id: i32) -> Result<u32> {
        self.neg_err(unsafe { ffi::fluid_synth_get_bank_offset(self.handle, sfont_id) })
            .map(|val| val as _)
    }
}
