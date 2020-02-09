use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use crate::{ffi, Synth};

/**
Chorus type
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum ChorusMode {
    Sine = ffi::fluid_chorus_mod_FLUID_CHORUS_MOD_SINE,
    Triangle = ffi::fluid_chorus_mod_FLUID_CHORUS_MOD_TRIANGLE,
}

impl Default for ChorusMode {
    fn default() -> Self {
        ChorusMode::Sine
    }
}

/**
Chorus parameters
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChorusParams {
    pub nr: u32,
    pub level: f64,
    /// Speed in Hz
    pub speed: f64,
    /// Depth in mS
    pub depth: f64,
    /// Mode
    pub mode: ChorusMode,
}

impl Default for ChorusParams {
    fn default() -> Self {
        Self {
            nr: ffi::FLUID_CHORUS_DEFAULT_N,
            level: ffi::FLUID_CHORUS_DEFAULT_LEVEL,
            speed: ffi::FLUID_CHORUS_DEFAULT_SPEED,
            depth: ffi::FLUID_CHORUS_DEFAULT_DEPTH,
            mode: ChorusMode::default(),
        }
    }
}

/**
Chorus
 */
impl Synth {
    /**
    Set up the chorus. It should be turned on with Synth::chorus_on().
    If faulty parameters are given, all new settings are discarded.
    Keep in mind, that the needed CPU time is proportional to `nr`.
     */
    pub fn set_chorus_params(&self, nr: u32, level: f64, speed: f64, depth: f64, mode: ChorusMode) {
        unsafe { ffi::fluid_synth_set_chorus(
            self.handle, nr as i32, level, speed, depth, mode as i32); }
    }

    /**
    Set up the chorus. It should be turned on with Synth::chorus_on().
    If faulty parameters are given, all new settings are discarded.
    Keep in mind, that the needed CPU time is proportional to `nr`.
     */
    pub fn set_chorus(&self, params: &ChorusParams) {
        self.set_chorus_params(params.nr, params.level, params.speed, params.depth, params.mode);
    }

    /** Turn on/off the built-in chorus unit */
    pub fn set_chorus_on(&self, on: bool) {
        unsafe { ffi::fluid_synth_set_chorus_on(self.handle, on as _); }
    }

    /**
    Query the current chorus nr
     */
    pub fn get_chorus_nr(&self) -> u32 {
        unsafe { ffi::fluid_synth_get_chorus_nr(self.handle) as _ }
    }

    /**
    Query the current chorus level
     */
    pub fn get_chorus_level(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_chorus_level(self.handle) as _ }
    }

    /**
    Query the current chorus speed (Hz)
     */
    pub fn get_chorus_speed(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_chorus_speed_Hz(self.handle) as _ }
    }

    /**
    Query the current chorus depth (mS)
     */
    pub fn get_chorus_depth(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_chorus_depth_ms(self.handle) as _ }
    }

    /**
    Query the current chorus mode
     */
    pub fn get_chorus_mode(&self) -> ChorusMode {
        ChorusMode::from_i32(unsafe { ffi::fluid_synth_get_chorus_type(self.handle) }).unwrap()
    }

    /**
    Query the current chorus params
     */
    pub fn get_chorus(&self) -> ChorusParams {
        ChorusParams {
            nr: self.get_chorus_nr(),
            level: self.get_chorus_level(),
            speed: self.get_chorus_speed(),
            depth: self.get_chorus_depth(),
            mode: self.get_chorus_mode(),
        }
    }
}
