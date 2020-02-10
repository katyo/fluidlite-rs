use std::{
    mem::MaybeUninit,
    fmt::{Display, Formatter, Result as FmtResult},
};
use crate::ffi;

/**
The library version info
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.major.fmt(f)?;
        '.'.fmt(f)?;
        self.minor.fmt(f)?;
        '.'.fmt(f)?;
        self.micro.fmt(f)
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, micro: u32) -> Self {
        Self { major, minor, micro }
    }

    pub fn get() -> Version {
        let mut major = MaybeUninit::uninit();
        let mut minor = MaybeUninit::uninit();
        let mut micro = MaybeUninit::uninit();

        unsafe { ffi::fluid_version(
            major.as_mut_ptr(),
            minor.as_mut_ptr(),
            micro.as_mut_ptr(),
        ); }

        Version::new(
            unsafe { major.assume_init() as _ },
            unsafe { minor.assume_init() as _ },
            unsafe { micro.assume_init() as _ },
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Version;

    #[test]
    fn version() {
        let ver = Version::get();

        assert_eq!(ver, Version::new(1, 2, 0));
        assert_eq!(ver.to_string(), "1.2.0");
    }
}
