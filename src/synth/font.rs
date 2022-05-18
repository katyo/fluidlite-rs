use crate::{ffi, option_from_ptr, Chan, Error, FontId, FontRef, PresetRef, Result, Status, Synth};
use std::{ffi::CString, marker::PhantomData, path::Path, ptr::NonNull};

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
     */
    pub fn sfload<P: AsRef<Path>>(&self, filename: P, reset_presets: bool) -> Result<FontId> {
        let filename = filename.as_ref().to_str().ok_or(Error::Path)?;
        let filename = CString::new(filename).map_err(|_| Error::Path)?;

        self.neg_err(unsafe {
            ffi::fluid_synth_sfload(self.handle.as_ptr(), filename.as_ptr(), reset_presets as _)
        })
        .map(|id| id as _)
    }

    /**
    Reload a SoundFont. The reloaded SoundFont retains its ID and
    index on the stack.
     */
    pub fn sfreload(&self, id: FontId) -> Result<FontId> {
        self.neg_err(unsafe { ffi::fluid_synth_sfreload(self.handle.as_ptr(), id as _) })
            .map(|id| id as _)
    }

    /**
    Removes a SoundFont from the stack and deallocates it.
     */
    pub fn sfunload(&self, id: FontId, reset_presets: bool) -> Status {
        self.zero_ok(unsafe {
            ffi::fluid_synth_sfunload(self.handle.as_ptr(), id, reset_presets as _)
        })
    }

    /**
    Count the number of loaded SoundFonts.
     */
    pub fn sfcount(&self) -> u32 {
        unsafe { ffi::fluid_synth_sfcount(self.handle.as_ptr()) as _ }
    }

    /**
    Get a SoundFont. The SoundFont is specified by its index on the
    stack. The top of the stack has index zero.

    - `num` The number of the SoundFont (0 <= num < sfcount)
     */
    pub fn get_sfont(&self, num: u32) -> Option<FontRef<'_>> {
        option_from_ptr(unsafe { ffi::fluid_synth_get_sfont(self.handle.as_ptr(), num) })
            .map(|ptr| unsafe { FontRef::from_ptr(ptr) })
    }

    /**
    Get an iterator over loaded SoundFonts.
     */
    pub fn sfont_iter(&self) -> FontIter<'_> {
        unsafe { FontIter::from_ptr(self.handle) }
    }

    /**
    Get a SoundFont. The SoundFont is specified by its ID.
     */
    pub fn get_sfont_by_id(&self, id: FontId) -> Option<FontRef<'_>> {
        option_from_ptr(unsafe { ffi::fluid_synth_get_sfont_by_id(self.handle.as_ptr(), id) })
            .map(|ptr| unsafe { FontRef::from_ptr(ptr) })
    }

    /**
    Remove a SoundFont that was previously added using
    fluid_synth_add_sfont(). The synthesizer does not delete the
    SoundFont; this is responsability of the caller.
     */
    pub fn remove_sfont(&self, sfont: FontRef<'_>) {
        unsafe {
            ffi::fluid_synth_remove_sfont(self.handle.as_ptr(), sfont.as_ptr());
        }
    }

    /*
    /**
    Add a SoundFont. The SoundFont will be put on top of
    the SoundFont stack.
     */
    pub fn add_sfont(&self, sfont: &SFont) -> Result<FontId> {
        self.neg_err(unsafe { ffi::fluid_synth_add_sfont(self.handle.as_ptr(), sfont.as_ptr()) })
    }
     */

    /**
    Get the preset of a channel
     */
    pub fn get_channel_preset(&self, chan: Chan) -> Option<PresetRef<'_>> {
        option_from_ptr(unsafe {
            ffi::fluid_synth_get_channel_preset(self.handle.as_ptr(), chan as _)
        })
        .map(|ptr| unsafe { PresetRef::from_ptr(ptr) })
    }

    /**
    Offset the bank numbers in a SoundFont.
    Returns -1 if an error occured (out of memory or negative offset)
     */
    pub fn set_bank_offset(&self, sfont_id: FontId, offset: u32) -> Status {
        self.zero_ok(unsafe {
            ffi::fluid_synth_set_bank_offset(self.handle.as_ptr(), sfont_id as _, offset as _)
        })
    }

    /**
    Get the offset of the bank numbers in a SoundFont.
     */
    pub fn get_bank_offset(&self, sfont_id: FontId) -> Result<u32> {
        self.neg_err(unsafe {
            ffi::fluid_synth_get_bank_offset(self.handle.as_ptr(), sfont_id as _)
        })
        .map(|val| val as _)
    }
}

#[cfg(test)]
mod test {
    use crate::{IsFont, IsPreset, Settings, Synth};

    #[test]
    fn font_and_preset() {
        let synth = Synth::new(Settings::new().unwrap()).unwrap();

        assert_eq!(synth.sfcount(), 0);

        synth.sfload("sf_/Boomwhacker.sf2", true).unwrap();

        assert_eq!(synth.sfcount(), 1);

        let font = synth.get_sfont(0).unwrap();

        assert_eq!(font.get_id(), 1);
        assert_eq!(font.get_name().unwrap(), "sf_/Boomwhacker.sf2");

        let preset = font.get_preset(0, 0).unwrap();

        assert_eq!(preset.get_name().unwrap(), "Boomwhacker");
        assert_eq!(preset.get_banknum().unwrap(), 0);
        assert_eq!(preset.get_num().unwrap(), 0);
    }
}

/**
The iterator over loaded SoundFonts.
 */
pub struct FontIter<'a> {
    handle: NonNull<ffi::fluid_synth_t>,
    phantom: PhantomData<&'a ()>,
    font_no: u32,
}

impl<'a> FontIter<'a> {
    unsafe fn from_ptr(handle: NonNull<ffi::fluid_synth_t>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            font_no: 0,
        }
    }
}

impl<'a> Iterator for FontIter<'a> {
    type Item = FontRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let font = option_from_ptr(unsafe {
            ffi::fluid_synth_get_sfont(self.handle.as_ptr(), self.font_no)
        })
        .map(|ptr| unsafe { FontRef::from_ptr(ptr) });
        if font.is_some() {
            self.font_no += 1;
        }
        font
    }
}
