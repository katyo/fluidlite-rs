use crate::{ffi, Synth, Status};

/// The trait which implements samples data buffer interface
pub trait IsSamples {
    fn write_samples(self, synth: &Synth) -> Status;
}

impl IsSamples for &mut [i16] {
    /// Write samples interleaved
    fn write_samples(self, synth: &Synth) -> Status {
        let len = self.len() / 2;
        unsafe { synth.write_i16(
            len, self.as_mut_ptr(), 0, 2, self.as_mut_ptr(), 1, 2) }
    }
}

impl IsSamples for (&mut [i16], &mut [i16]) {
    /// Write samples non-interleaved
    fn write_samples(self, synth: &Synth) -> Status {
        let len = self.0.len().min(self.1.len());
        unsafe { synth.write_i16(
            len, self.0.as_mut_ptr(), 0, 1, self.1.as_mut_ptr(), 0, 1) }
    }
}

impl IsSamples for &mut [f32] {
    /// Write samples interleaved
    fn write_samples(self, synth: &Synth) -> Status {
        let len = self.len() / 2;
        unsafe { synth.write_f32(
            len, self.as_mut_ptr(), 0, 2, self.as_mut_ptr(), 1, 2) }
    }
}

impl IsSamples for (&mut [f32], &mut [f32]) {
    /// Write samples non-interleaved
    fn write_samples(self, synth: &Synth) -> Status {
        let len = self.0.len().min(self.1.len());
        unsafe { synth.write_f32(
            len, self.0.as_mut_ptr(), 0, 1, self.1.as_mut_ptr(), 0, 1) }
    }
}

/**
Synthesizer plugin
 */
impl Synth {
    /**
    Write sound samples to the sample data buffer
     */
    pub fn write<S: IsSamples>(&self, samples: S) -> Status {
        samples.write_samples(self)
    }

    /**
    Write samples as 16-bit signed integers

    __Note__: The `len` must corresponds to the lenghtes of buffers.
     */
    #[inline]
    pub unsafe fn write_i16(&self,
                            len: usize,
                            lbuf: *mut i16, loff: u32, lincr: u32,
                            rbuf: *mut i16, roff: u32, rincr: u32) -> Status {
        self.zero_ok(ffi::fluid_synth_write_s16(
            self.handle, len as _,
				    lbuf as _, loff as _, lincr as _,
				    rbuf as _, roff as _, rincr as _))
    }

    /**
    Write samples as 32-bit floating-point numbers

    __Note__: The `len` must corresponds to the lenghtes of buffers.
     */
    #[inline]
    pub unsafe fn write_f32(&self,
                            len: usize,
                            lbuf: *mut f32, loff: u32, lincr: u32,
                            rbuf: *mut f32, roff: u32, rincr: u32) -> Status {
        self.zero_ok(ffi::fluid_synth_write_float(
            self.handle, len as _,
				    lbuf as _, loff as _, lincr as _,
				    rbuf as _, roff as _, rincr as _))
    }
}
