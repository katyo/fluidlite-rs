use crate::{ffi, Synth, Status};

/// The trait which implements samples data buffer interface
pub trait IsSamples {
    fn write_samples(self, handle: *mut ffi::fluid_synth_t) -> i32;
}

impl IsSamples for &mut [i16] {
    /// Write samples interleaved
    fn write_samples(self, handle: *mut ffi::fluid_synth_t) -> i32 {
        let len = self.len() / 2;
        unsafe {
            ffi::fluid_synth_write_s16(handle, len as _,
				                               self.as_mut_ptr() as _, 0, 2,
				                               self.as_mut_ptr() as _, 1, 2)
        }
    }
}

impl IsSamples for (&mut [i16], &mut [i16]) {
    /// Write samples non-interleaved
    fn write_samples(self, handle: *mut ffi::fluid_synth_t) -> i32 {
        let len = self.0.len().min(self.1.len());
        unsafe {
            ffi::fluid_synth_write_s16(handle, len as _,
				                               self.0.as_mut_ptr() as _, 0, 1,
				                               self.1.as_mut_ptr() as _, 0, 1)
        }
    }
}

impl IsSamples for &mut [f32] {
    /// Write samples interleaved
    fn write_samples(self, handle: *mut ffi::fluid_synth_t) -> i32 {
        let len = self.len() / 2;
        unsafe {
            ffi::fluid_synth_write_float(handle, len as _,
				                                 self.as_mut_ptr() as _, 0, 2,
				                                 self.as_mut_ptr() as _, 1, 2)
        }
    }
}

impl IsSamples for (&mut [f32], &mut [f32]) {
    /// Write samples non-interleaved
    fn write_samples(self, handle: *mut ffi::fluid_synth_t) -> i32 {
        let len = self.0.len().min(self.1.len());
        unsafe {
            ffi::fluid_synth_write_float(handle, len as _,
				                                 self.0.as_mut_ptr() as _, 0, 1,
				                                 self.1.as_mut_ptr() as _, 0, 1)
        }
    }
}

/**
Synthesizer plugin
 */
impl Synth {
    /**
    Write sound samples to sample data buffer
     */
    pub fn write<S: IsSamples>(&self, samples: S) -> Status {
        self.zero_ok(samples.write_samples(self.handle))
    }
}
