use std::mem::MaybeUninit;
use crate::{Synth, Result, Status, Chan, Key, Vel, Ctrl, Val, Prog, Bank, FontId, PresetId, ffi};

/**
MIDI channel messages
 */
impl Synth {
    /**
    Send a noteon message.
     */
    pub fn note_on(&self, chan: Chan, key: Key, vel: Vel) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_noteon(self.handle, chan as _, key as _, vel as _) })
    }

    /**
    Send a noteoff message.
     */
    pub fn note_off(&self, chan: Chan, key: Key) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_noteoff(self.handle, chan as _, key as _) })
    }

    /**
    Send a control change message.
     */
    pub fn cc(&self, chan: Chan, ctrl: Ctrl, val: Val) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_cc(self.handle, chan as _, ctrl as _, val as _) })
    }

    /**
    Get a control value.
     */
    pub fn get_cc(&self, chan: Chan, ctrl: Ctrl) -> Result<Val> {
        let mut val = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_cc(self.handle, chan as _, ctrl as _, val.as_mut_ptr()) })
            .map(|_| unsafe { val.assume_init() as _ })
    }

    /**
    Send a pitch bend message.
     */
    pub fn pitch_bend(&self, chan: Chan, val: Val) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_pitch_bend(self.handle, chan as _, val as _) })
    }

    /**
    Get the pitch bend value.
     */
    pub fn get_pitch_bend(&self, chan: Chan) -> Result<Val> {
        let mut pitch_bend = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_pitch_bend(self.handle, chan as _, pitch_bend.as_mut_ptr()) })
            .map(|_| unsafe { pitch_bend.assume_init() as _ })
    }

    /**
    Set the pitch wheel sensitivity.
     */
    pub fn pitch_wheel_sens(&self, chan: Chan, val: Val) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_pitch_wheel_sens(self.handle, chan as _, val as _) })
    }

    /**
    Get the pitch wheel sensitivity.
     */
    pub fn get_pitch_wheel_sens(&self, chan: Chan) -> Result<Val> {
        let mut val = MaybeUninit::uninit();

        self.zero_ok(unsafe { ffi::fluid_synth_get_pitch_wheel_sens(self.handle, chan as _, val.as_mut_ptr()) })
            .map(|_| unsafe { val.assume_init() as _ })
    }

    /**
    Send a program change message.
     */
    pub fn program_change(&self, chan: Chan, prog: Prog) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_program_change(self.handle, chan as _, prog as _) })
    }

    pub fn channel_pressure(&self, chan: Chan, val: Val) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_channel_pressure(self.handle, chan as _, val as _) })
    }

    /**
    Select a bank.
     */
    pub fn bank_select(&self, chan: Chan, bank: Bank) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_bank_select(self.handle, chan as _, bank) })
    }

    /**
    Select a sfont.
     */
    pub fn sfont_select(&self, chan: Chan, sfont_id: FontId) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_sfont_select(self.handle, chan as _, sfont_id) })
    }

    /**
    Select a preset for a channel. The preset is specified by the
    SoundFont ID, the bank number, and the preset number. This
    allows any preset to be selected and circumvents preset masking
    due to previously loaded SoundFonts on the SoundFont stack.
     */
    pub fn program_select(&self, chan: Chan, sfont_id: FontId, bank_num: Bank, preset_num: PresetId) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_program_select(self.handle, chan as _, sfont_id, bank_num, preset_num) })
    }

    /**
    Returns the program, bank, and SoundFont number of the preset on a given channel.
     */
    pub fn get_program(&self, chan: Chan) -> Result<(FontId, Bank, PresetId)> {
        let mut sfont_id = MaybeUninit::uninit();
        let mut bank_num = MaybeUninit::uninit();
        let mut preset_num = MaybeUninit::uninit();

        self.zero_ok(unsafe {
            ffi::fluid_synth_get_program(self.handle, chan as _,
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
