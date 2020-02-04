use std::mem::MaybeUninit;
use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub micro: i32,
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
        major: unsafe { major.assume_init() },
        minor: unsafe { minor.assume_init() },
        micro: unsafe { micro.assume_init() },
    }
}
