use crate::{ffi, Synth};

/**
Reverb parameters
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReverbParams {
    pub roomsize: f64,
    pub damp: f64,
    pub width: f64,
    pub level: f64,
}

impl Default for ReverbParams {
    fn default() -> Self {
        Self {
            roomsize: ffi::FLUID_REVERB_DEFAULT_ROOMSIZE,
            damp: ffi::FLUID_REVERB_DEFAULT_DAMP,
            width: ffi::FLUID_REVERB_DEFAULT_WIDTH,
            level: ffi::FLUID_REVERB_DEFAULT_LEVEL,
        }
    }
}

/**
Reverb
 */
impl Synth {
    /**
    Set the parameters for the built-in reverb unit
     */
    pub fn set_reverb_params(&self, roomsize: f64, damp: f64, width: f64, level: f64) {
        unsafe {
            ffi::fluid_synth_set_reverb(self.handle.as_ptr(), roomsize, damp, width, level);
        }
    }

    /**
    Set the parameters for the built-in reverb unit
     */
    pub fn set_reverb(&self, params: &ReverbParams) {
        self.set_reverb_params(params.roomsize, params.damp, params.width, params.level);
    }

    /**
    Turn on/off the built-in reverb unit
     */
    pub fn set_reverb_on(&self, on: bool) {
        unsafe {
            ffi::fluid_synth_set_reverb_on(self.handle.as_ptr(), on as _);
        }
    }

    /**
    Query the current reverb room size
     */
    pub fn get_reverb_roomsize(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_reverb_roomsize(self.handle.as_ptr()) }
    }

    /**
    Query the current reverb dumping
     */
    pub fn get_reverb_damp(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_reverb_damp(self.handle.as_ptr()) }
    }

    /**
    Query the current reverb level
     */
    pub fn get_reverb_level(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_reverb_level(self.handle.as_ptr()) }
    }

    /**
    Query the current reverb width
     */
    pub fn get_reverb_width(&self) -> f64 {
        unsafe { ffi::fluid_synth_get_reverb_width(self.handle.as_ptr()) }
    }

    /**
    Query the current reverb params
     */
    pub fn get_reverb(&self) -> ReverbParams {
        ReverbParams {
            roomsize: self.get_reverb_roomsize(),
            damp: self.get_reverb_damp(),
            level: self.get_reverb_level(),
            width: self.get_reverb_width(),
        }
    }
}
