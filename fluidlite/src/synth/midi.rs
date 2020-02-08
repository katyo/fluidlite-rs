use std::mem::MaybeUninit;
use crate::{Synth, Result, Status, ffi};

/**
MIDI channel messages
 */
impl Synth {
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
}
