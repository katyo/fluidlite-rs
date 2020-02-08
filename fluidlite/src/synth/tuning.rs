use std::{
    mem::MaybeUninit,
    ffi::{CStr, CString},
};
use crate::{ffi, Synth, Result, Status};

/**
 * Tuning
 */
impl Synth {
    /**
    Create a new key-based tuning with given name, number, and
    pitches. The array 'pitches' should have length 128 and contains
    the pitch in cents of every key in cents. However, if 'pitches' is
    NULL, a new tuning is created with the well-tempered scale.

    \param synth The synthesizer object
    \param tuning_bank The tuning bank number [0-127]
    \param tuning_prog The tuning program number [0-127]
    \param name The name of the tuning
    \param pitch The array of pitch values. The array length has to be 128.
     */
    pub fn create_key_tuning<S: AsRef<str>>(&self, tuning_bank: u32, tuning_prog: u32, name: S, pitch: &[f64; 128]) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe { ffi::fluid_synth_create_key_tuning(
            self.handle, tuning_bank as _, tuning_prog as _, name.as_ptr(), pitch.as_ptr() as _) })
    }

    /**
    Create a new octave-based tuning with given name, number, and
    pitches.  The array 'pitches' should have length 12 and contains
    derivation in cents from the well-tempered scale. For example, if
    pitches[0] equals -33, then the C-keys will be tuned 33 cents
    below the well-tempered C.
    \param synth The synthesizer object
    \param tuning_bank The tuning bank number [0-127]
    \param tuning_prog The tuning program number [0-127]
    \param name The name of the tuning
    \param pitch The array of pitch derivations. The array length has to be 12.
     */
    pub fn create_octave_tuning<S: AsRef<str>>(&self, tuning_bank: u32, tuning_prog: u32, name: S, pitch: &[f64; 12]) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe { ffi::fluid_synth_create_octave_tuning(
            self.handle, tuning_bank as _, tuning_prog as _, name.as_ptr(), pitch.as_ptr()) })
    }

    pub fn activate_octave_tuning<S: AsRef<str>>(&self, bank: u32, prog: u32, name: S, pitch: &[f64; 12], apply: bool) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe { ffi::fluid_synth_activate_octave_tuning(
            self.handle, bank as _, prog as _, name.as_ptr(), pitch.as_ptr(), apply as _) })
    }

    /**
    Request a note tuning changes. Both they 'keys' and 'pitches'
    arrays should be of length 'num_pitches'. If 'apply' is non-zero,
    the changes should be applied in real-time, i.e. sounding notes
    will have their pitch updated. 'APPLY' IS CURRENTLY IGNORED. The
    changes will be available for newly triggered notes only.
    \param synth The synthesizer object
    \param tuning_bank The tuning bank number [0-127]
    \param tuning_prog The tuning program number [0-127]
    \param len The length of the keys and pitch arrays
    \param keys The array of keys values.
    \param pitch The array of pitch values.
    \param apply Flag to indicate whether to changes should be applied in real-time.
     */
    pub fn tune_notes<K, P>(&self, tuning_bank: u32, tuning_prog: u32, keys: K, pitch: P, apply: bool) -> Status
    where
        K: AsRef<[u32]>,
        P: AsRef<[f64]>,
    {
        let keys = keys.as_ref();
        let pitch = pitch.as_ref();
        let len = keys.len().min(pitch.len());
        self.zero_ok(unsafe { ffi::fluid_synth_tune_notes(
            self.handle, tuning_bank as _, tuning_prog as _, len as _, keys.as_ptr() as _, pitch.as_ptr() as _, apply as _) })
    }

    /**
    Select a tuning for a channel.
    \param synth The synthesizer object
    \param chan The channel number [0-max channels]
    \param tuning_bank The tuning bank number [0-127]
    \param tuning_prog The tuning program number [0-127]
     */
    pub fn select_tuning(&self, chan: u32, tuning_bank: u32, tuning_prog: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_select_tuning(self.handle, chan as _, tuning_bank as _, tuning_prog as _) })
    }

    pub fn activate_tuning(&self, chan: u32, bank: u32, prog: u32, apply: bool) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_activate_tuning(self.handle, chan as _, bank as _, prog as _, apply as _) })
    }

    /**
    Set the tuning to the default well-tempered tuning on a channel.
    \param synth The synthesizer object
    \param chan The channel number [0-max channels]
     */
    pub fn reset_tuning(&self, chan: u32) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_reset_tuning(self.handle, chan as _) })
    }

    /**
    Start the iteration throught the list of available tunings.
    \param synth The synthesizer object
     */
    pub fn tuning_iteration_start(&self) {
        unsafe { ffi::fluid_synth_tuning_iteration_start(self.handle); }
    }

    /**
    Get the next tuning in the iteration. This functions stores the
    bank and program number of the next tuning in the pointers given as
    arguments.
    \param synth The synthesizer object
    \param bank Pointer to an int to store the bank number
    \param prog Pointer to an int to store the program number
    \returns 1 if there is a next tuning, 0 otherwise
     */
    pub fn iteration_next(&self) -> (u32, u32, bool) {
        let mut bank = MaybeUninit::uninit();
        let mut prog = MaybeUninit::uninit();
        let next = unsafe { ffi::fluid_synth_tuning_iteration_next(self.handle, bank.as_mut_ptr(), prog.as_mut_ptr()) };
        (
            unsafe { bank.assume_init() as _ },
            unsafe { prog.assume_init() as _ },
            next != 0,
        )
    }

    /**
    Dump the data of a tuning. This functions stores the name and
    pitch values of a tuning in the pointers given as arguments. Both
    name and pitch can be NULL is the data is not needed.
    \param synth The synthesizer object
    \param bank The tuning bank number [0-127]
    \param prog The tuning program number [0-127]
    \param name Pointer to a buffer to store the name
    \param len The length of the name buffer
    \param pitch Pointer to buffer to store the pitch values
     */
    pub fn tuning_dump(&self, bank: u32, prog: u32) -> Result<(String, [f64; 128])> {
        const NAME_LEN: usize = 128;
        let mut name = MaybeUninit::<[u8; NAME_LEN]>::uninit();
        let mut pitch = MaybeUninit::<[f64; 128]>::uninit();
        self.zero_ok(unsafe { ffi::fluid_synth_tuning_dump(
            self.handle, bank as _, prog as _, name.as_mut_ptr() as _, NAME_LEN as _, pitch.as_mut_ptr() as _) })?;
        Ok((
            (unsafe { CStr::from_ptr(name.as_ptr() as _) }).to_str().unwrap().into(),
            unsafe { pitch.assume_init() },
        ))
    }
}
