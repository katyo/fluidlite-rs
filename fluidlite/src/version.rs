use std::mem::MaybeUninit;
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
}

pub fn version() -> Version {
    let mut major = MaybeUninit::uninit();
    let mut minor = MaybeUninit::uninit();
    let mut micro = MaybeUninit::uninit();

    unsafe { ffi::fluid_version(
        major.as_mut_ptr(),
        minor.as_mut_ptr(),
        micro.as_mut_ptr(),
    ); }

    Version {
        major: unsafe { major.assume_init() as _ },
        minor: unsafe { minor.assume_init() as _ },
        micro: unsafe { micro.assume_init() as _ },
    }
}
