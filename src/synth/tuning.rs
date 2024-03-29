use crate::{ffi, Bank, Chan, Prog, Result, Status, Synth};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
    ptr::{null_mut, NonNull},
};

/**
 * Tuning
 */
impl Synth {
    /**
    Create a new key-based tuning with given name, number, and
    pitches. The array 'pitches' should have length 128 and contains
    the pitch in cents of every key in cents. However, if 'pitches' is
    NULL, a new tuning is created with the well-tempered scale.
     */
    pub fn create_key_tuning<S: AsRef<str>>(
        &self,
        tuning_bank: Bank,
        tuning_prog: Prog,
        name: S,
        pitch: &[f64; 128],
    ) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe {
            ffi::fluid_synth_create_key_tuning(
                self.handle.as_ptr(),
                tuning_bank as _,
                tuning_prog as _,
                name.as_ptr(),
                pitch.as_ptr() as _,
            )
        })
    }

    /**
    Create a new octave-based tuning with given name, number, and
    pitches.  The array 'pitches' should have length 12 and contains
    derivation in cents from the well-tempered scale. For example, if
    pitches[0] equals -33, then the C-keys will be tuned 33 cents
    below the well-tempered C.
     */
    pub fn create_octave_tuning<S: AsRef<str>>(
        &self,
        tuning_bank: Bank,
        tuning_prog: Prog,
        name: S,
        pitch: &[f64; 12],
    ) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe {
            ffi::fluid_synth_create_octave_tuning(
                self.handle.as_ptr(),
                tuning_bank as _,
                tuning_prog as _,
                name.as_ptr(),
                pitch.as_ptr(),
            )
        })
    }

    pub fn activate_octave_tuning<S: AsRef<str>>(
        &self,
        bank: Bank,
        prog: Prog,
        name: S,
        pitch: &[f64; 12],
        apply: bool,
    ) -> Status {
        let name = CString::new(name.as_ref()).unwrap();
        self.zero_ok(unsafe {
            ffi::fluid_synth_activate_octave_tuning(
                self.handle.as_ptr(),
                bank as _,
                prog as _,
                name.as_ptr(),
                pitch.as_ptr(),
                apply as _,
            )
        })
    }

    /**
    Request a note tuning changes. Both they 'keys' and 'pitches'
    arrays should be of length 'num_pitches'. If 'apply' is non-zero,
    the changes should be applied in real-time, i.e. sounding notes
    will have their pitch updated. 'APPLY' IS CURRENTLY IGNORED. The
    changes will be available for newly triggered notes only.
     */
    pub fn tune_notes<K, P>(
        &self,
        tuning_bank: Bank,
        tuning_prog: Prog,
        keys: K,
        pitch: P,
        apply: bool,
    ) -> Status
    where
        K: AsRef<[u32]>,
        P: AsRef<[f64]>,
    {
        let keys = keys.as_ref();
        let pitch = pitch.as_ref();
        let len = keys.len().min(pitch.len());
        self.zero_ok(unsafe {
            ffi::fluid_synth_tune_notes(
                self.handle.as_ptr(),
                tuning_bank as _,
                tuning_prog as _,
                len as _,
                keys.as_ptr() as _,
                pitch.as_ptr() as _,
                apply as _,
            )
        })
    }

    /**
    Select a tuning for a channel.
     */
    pub fn select_tuning(&self, chan: Chan, tuning_bank: Bank, tuning_prog: Prog) -> Status {
        self.zero_ok(unsafe {
            ffi::fluid_synth_select_tuning(
                self.handle.as_ptr(),
                chan as _,
                tuning_bank as _,
                tuning_prog as _,
            )
        })
    }

    pub fn activate_tuning(&self, chan: Chan, bank: Bank, prog: Prog, apply: bool) -> Status {
        self.zero_ok(unsafe {
            ffi::fluid_synth_activate_tuning(
                self.handle.as_ptr(),
                chan as _,
                bank as _,
                prog as _,
                apply as _,
            )
        })
    }

    /**
    Set the tuning to the default well-tempered tuning on a channel.
     */
    pub fn reset_tuning(&self, chan: Chan) -> Status {
        self.zero_ok(unsafe { ffi::fluid_synth_reset_tuning(self.handle.as_ptr(), chan as _) })
    }

    /**
    Get the iterator throught the list of available tunings.
     */
    pub fn tuning_iter(&self) -> TuningIter<'_> {
        unsafe { TuningIter::from_ptr(self.handle) }
    }

    /**
    Dump the data of a tuning.

    This function returns both the name and pitch values of a tuning.
     */
    pub fn tuning_dump(&self, bank: Bank, prog: Prog) -> Result<(String, [f64; 128])> {
        const NAME_LEN: usize = 128;

        let mut name = MaybeUninit::<[u8; NAME_LEN]>::uninit();
        let mut pitch = MaybeUninit::<[f64; 128]>::uninit();

        self.zero_ok(unsafe {
            ffi::fluid_synth_tuning_dump(
                self.handle.as_ptr(),
                bank as _,
                prog as _,
                name.as_mut_ptr() as _,
                NAME_LEN as _,
                pitch.as_mut_ptr() as _,
            )
        })?;
        Ok((
            (unsafe { CStr::from_ptr(name.as_ptr() as _) })
                .to_str()
                .unwrap()
                .into(),
            unsafe { pitch.assume_init() },
        ))
    }

    /**
    Dump the data of a tuning.

    This function returns the only name of a tuning.
     */
    pub fn tuning_dump_name(&self, bank: Bank, prog: Prog) -> Result<String> {
        const NAME_LEN: usize = 128;

        let mut name = MaybeUninit::<[u8; NAME_LEN]>::uninit();

        self.zero_ok(unsafe {
            ffi::fluid_synth_tuning_dump(
                self.handle.as_ptr(),
                bank as _,
                prog as _,
                name.as_mut_ptr() as _,
                NAME_LEN as _,
                null_mut(),
            )
        })?;
        Ok((unsafe { CStr::from_ptr(name.as_ptr() as _) })
            .to_str()
            .unwrap()
            .into())
    }

    /**
    Dump the data of a tuning.

    This function returns the only pitch values of a tuning.
     */
    pub fn tuning_dump_pitch(&self, bank: Bank, prog: Prog) -> Result<[f64; 128]> {
        let mut pitch = MaybeUninit::<[f64; 128]>::uninit();

        self.zero_ok(unsafe {
            ffi::fluid_synth_tuning_dump(
                self.handle.as_ptr(),
                bank as _,
                prog as _,
                null_mut(),
                0,
                pitch.as_mut_ptr() as _,
            )
        })?;
        Ok(unsafe { pitch.assume_init() })
    }
}

/**
The iterator over tunings
 */
pub struct TuningIter<'a> {
    handle: NonNull<ffi::fluid_synth_t>,
    phantom: PhantomData<&'a ()>,
    init: bool,
    next: bool,
}

impl<'a> TuningIter<'a> {
    unsafe fn from_ptr(handle: NonNull<ffi::fluid_synth_t>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            init: true,
            next: true,
        }
    }
}

impl<'a> Iterator for TuningIter<'a> {
    type Item = (Bank, Prog);

    fn next(&mut self) -> Option<Self::Item> {
        if self.init {
            self.init = false;
            unsafe {
                ffi::fluid_synth_tuning_iteration_start(self.handle.as_ptr());
            }
        }
        if self.next {
            let mut bank = MaybeUninit::uninit();
            let mut prog = MaybeUninit::uninit();
            self.next = 0
                != unsafe {
                    ffi::fluid_synth_tuning_iteration_next(
                        self.handle.as_ptr(),
                        bank.as_mut_ptr(),
                        prog.as_mut_ptr(),
                    )
                };

            Some((unsafe { bank.assume_init() as _ }, unsafe {
                prog.assume_init() as _
            }))
        } else {
            None
        }
    }
}
