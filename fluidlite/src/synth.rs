use std::{
    mem::MaybeUninit,
    ffi::{CStr, CString},
    path::Path,
};
use crate::{ffi, Settings, Result, Status};

/**
The synth object
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

    /**
    Send a noteon message.

    Returns `true` if no error occurred, `false` otherwise.
     */
    pub fn noteon(&self, chan: i32, key: i32, vel: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_noteon(self.handle, chan, key, vel) })
    }

    /**
    Send a noteoff message.

    Returns `true` if no error occurred, `false` otherwise.
     */
    pub fn noteoff(&self, chan: i32, key: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_noteoff(self.handle, chan, key) })
    }

    /**
    Send a control change message.

    Returns `true` if no error occurred, `false` otherwise.
     */
    pub fn cc(&self, chan: i32, ctrl: i32, val: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_cc(self.handle, chan, ctrl, val) })
    }

    /**
    Get a control value.

    Returns `Some(value)` if no error occurred, `None` otherwise.
     */
    pub fn get_cc(&self, chan: i32, ctrl: i32) -> Result<i32> {
        let mut val = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_cc(self.handle, chan, ctrl, val.as_mut_ptr()) })
            .map(|_| unsafe { val.assume_init() })
    }

    /**
    Send a pitch bend message.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn pitch_bend(&self, chan: i32, val: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_pitch_bend(self.handle, chan, val) })
    }

    /**
    Get the pitch bend value.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn get_pitch_bend(&self, chan: i32) -> Result<i32> {
        let mut pitch_bend = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_pitch_bend(self.handle, chan, pitch_bend.as_mut_ptr()) })
            .map(|_| unsafe { pitch_bend.assume_init() })
    }

    /**
    Set the pitch wheel sensitivity.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn pitch_wheel_sens(&self, chan: i32, val: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_pitch_wheel_sens(self.handle, chan, val) })
    }

    /**
    Get the pitch wheel sensitivity.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn get_pitch_wheel_sens(&self, chan: i32) -> Result<i32> {
        let mut val = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_pitch_wheel_sens(self.handle, chan, val.as_mut_ptr()) })
            .map(|_| unsafe { val.assume_init() })
    }

    /**
    Send a program change message.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn program_change(&self, chan: i32, program: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_program_change(self.handle, chan, program) })
    }

    pub fn channel_pressure(&self, chan: i32, val: i32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_channel_pressure(self.handle, chan, val) })
    }

    /*pub fn sysex(&self, data: &str, dryrun: bool) -> bool {
        if 0 == unsafe { fluid_synth_sysex(self.handle, const char *data, int len,
                                           char *response, int *response_len, int *handled, int dryrun) } {

        } else {
        }
    }*/

    /**
    Select a bank.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn bank_select(&self, chan: i32, bank: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_bank_select(self.handle, chan, bank) })
    }

    /**
    Select a sfont.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn sfont_select(&self, chan: i32, sfont_id: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_sfont_select(self.handle, chan, sfont_id) })
    }

    /**
    Select a preset for a channel. The preset is specified by the
    SoundFont ID, the bank number, and the preset number. This
    allows any preset to be selected and circumvents preset masking
    due to previously loaded SoundFonts on the SoundFont stack.
    \param synth The synthesizer
    \param chan The channel on which to set the preset
    \param sfont_id The ID of the SoundFont
    \param bank_num The bank number
    \param preset_num The preset number
    \return 0 if no errors occured, -1 otherwise
     */
    pub fn program_select(&self, chan: i32, sfont_id: u32, bank_num: u32, preset_num: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_program_select(self.handle, chan, sfont_id, bank_num, preset_num) })
    }

    /**
    Returns the program, bank, and SoundFont number of the preset on a given channel.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn get_program(&self, chan: i32) -> Result<(u32, u32, u32)> {
        let mut sfont_id = MaybeUninit::uninit();
        let mut bank_num = MaybeUninit::uninit();
        let mut preset_num = MaybeUninit::uninit();

        self.zero_ok(unsafe {
            ffi::fluid_synth_get_program(self.handle, chan,
			                                   sfont_id.as_mut_ptr(),
			                                   bank_num.as_mut_ptr(),
			                                   preset_num.as_mut_ptr())
        }).map(|_| unsafe { (
            sfont_id.assume_init(),
            bank_num.assume_init(),
            preset_num.assume_init(),
        ) })
    }

    /**
    Send a bank select and a program change to every channel to reinitialize the preset of the channel.

    This function is useful mainly after a SoundFont has been loaded, unloaded or reloaded.

    Returns 0 if no error occurred, -1 otherwise.
     */
    pub fn program_reset(&self) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_program_reset(self.handle) })
    }

    /**
    Send a reset.

    A reset turns all the notes off and resets the controller values.
     */
    pub fn system_reset(&self) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_system_reset(self.handle) })
    }

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
    pub fn sfload<P: AsRef<Path>>(&self, filename: P, reset_presets: bool) -> Result<i32> {
        let filename = filename.as_ref().to_str().ok_or_else(|| String::from("Invalid UTF-8"))?;
        let filename = CString::new(filename).map_err(|error| error.to_string())?;
        let reset_presets = if reset_presets { 1 } else { 0 };

        self.neg_err(unsafe { ffi::fluid_synth_sfload(self.handle, filename.as_ptr(), reset_presets) })
    }

    /**
    Get a textual representation of the last error
     */
    fn error(&self) -> String {
        let error = unsafe { ffi::fluid_synth_error(self.handle) };
        let error = unsafe { CStr::from_ptr(error) };
        error.to_str().unwrap().into()
    }

    fn neg_err(&self, ret: i32) -> Result<i32> {
        if ret < 0 {
            Err(self.error())
        } else {
            Ok(ret)
        }
    }

    fn zero_ok(&self, ret: i32) -> Status {
        if ret == 0 {
            Ok(())
        } else {
            Err(self.error())
        }
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        unsafe { ffi::delete_fluid_synth(self.handle); }
    }
}
