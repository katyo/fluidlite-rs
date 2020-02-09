use std::{
    marker::PhantomData,
    os::raw,
    mem::{MaybeUninit, transmute},
    ffi::{CString, CStr},
    ops::{Bound, RangeBounds},
};
use bitflags::bitflags;
use crate::{ffi, Result, result_from_ptr};

/**
The generic settings object
 */
#[repr(transparent)]
pub struct Settings {
    handle: *mut ffi::fluid_settings_t,
}

unsafe impl Send for Settings {}

/**
The settings reference
*/
#[repr(transparent)]
pub struct SettingsRef<'a> {
    handle: *mut ffi::fluid_settings_t,
    phantom: PhantomData<&'a ()>,
}

impl Drop for Settings {
    fn drop(&mut self) {
        unsafe { ffi::delete_fluid_settings(self.handle) }
    }
}

impl Settings {
    pub fn new() -> Result<Self> {
        result_from_ptr(unsafe { ffi::new_fluid_settings() })
            .map(|handle| Self { handle })
    }

    pub(crate) fn into_ptr(self) -> *mut ffi::fluid_settings_t {
        unsafe { transmute(self) }
    }

    pub(crate) fn from_ptr(handle: *mut ffi::fluid_settings_t) -> Self {
        Self { handle }
    }
}

impl<'a> SettingsRef<'a> {
    pub(crate) fn from_ptr(handle: *mut ffi::fluid_settings_t) -> Self {
        Self { handle, phantom: PhantomData }
    }
}

pub trait IsSettings {
    fn pick<S, T>(&self, name: S) -> Option<Setting<'_, T>>
    where
        S: Into<Vec<u8>>,
        T: IsSetting + ?Sized;

    fn str_<S>(&self, name: S) -> Option<Setting<'_, str>>
    where
        S: Into<Vec<u8>>;

    fn num<S>(&self, name: S) -> Option<Setting<'_, f64>>
    where
        S: Into<Vec<u8>>;

    fn int<S>(&self, name: S) -> Option<Setting<'_, i32>>
    where
        S: Into<Vec<u8>>;
}

mod private {
    use std::{
        ffi::CString,
        marker::PhantomData,
    };
    use super::{ffi, IsSettings, IsSetting, Setting, Settings, SettingsRef};

    pub trait HasHandle {
        type Handle;

        fn get_handle(&self) -> *mut Self::Handle;
    }

    impl<X> IsSettings for X where X: HasHandle<Handle = ffi::fluid_settings_t> {
        fn pick<S, T>(&self, name: S) -> Option<Setting<'_, T>>
        where
            S: Into<Vec<u8>>,
            T: IsSetting + ?Sized,
        {
            let handle = self.get_handle();
            let name = CString::new(name).ok()?;

            if T::TYPE == unsafe { ffi::fluid_settings_get_type(handle, name.as_ptr() as *const _) } {
                Some(Setting { handle, name, phantom: PhantomData })
            } else {
                None
            }
        }

        fn str_<S>(&self, name: S) -> Option<Setting<'_, str>>
        where
            S: Into<Vec<u8>>,
        {
            self.pick(name)
        }

        fn num<S>(&self, name: S) -> Option<Setting<'_, f64>>
        where
            S: Into<Vec<u8>>,
        {
            self.pick(name)
        }

        fn int<S>(&self, name: S) -> Option<Setting<'_, i32>>
        where
            S: Into<Vec<u8>>,
        {
            self.pick(name)
        }
    }

    impl HasHandle for Settings {
        type Handle = ffi::fluid_settings_t;

        fn get_handle(&self) -> *mut Self::Handle {
            self.handle
        }
    }

    impl<'a> HasHandle for SettingsRef<'a> {
        type Handle = ffi::fluid_settings_t;

        fn get_handle(&self) -> *mut Self::Handle {
            self.handle
        }
    }
}

pub trait IsSetting {
    const TYPE: ffi::fluid_types_enum;
}

impl IsSetting for str {
    const TYPE: ffi::fluid_types_enum = ffi::fluid_types_enum_FLUID_STR_TYPE;
}

impl IsSetting for f64 {
    const TYPE: ffi::fluid_types_enum = ffi::fluid_types_enum_FLUID_NUM_TYPE;
}

impl IsSetting for i32 {
    const TYPE: ffi::fluid_types_enum = ffi::fluid_types_enum_FLUID_INT_TYPE;
}

impl IsSetting for () {
    const TYPE: ffi::fluid_types_enum = ffi::fluid_types_enum_FLUID_SET_TYPE;
}

bitflags! {
    /**
    The setting hints
     */
    pub struct Hints: i32 {
        /**
        Hint BOUNDED_BELOW indicates that the LowerBound field
        of the FLUID_PortRangeHint should be considered meaningful. The
        value in this field should be considered the (inclusive) lower
        bound of the valid range. If SAMPLE_RATE is also
        specified then the value of LowerBound should be multiplied by the
        sample rate.
         */
        const BOUNDED_BELOW = ffi::FLUID_HINT_BOUNDED_BELOW as i32;

        /**
        Hint BOUNDED_ABOVE indicates that the UpperBound field
        of the FLUID_PortRangeHint should be considered meaningful. The
        value in this field should be considered the (inclusive) upper
        bound of the valid range. If SAMPLE_RATE is also
        specified then the value of UpperBound should be multiplied by the
        sample rate.
         */
        const BOUNDED_ABOVE = ffi::FLUID_HINT_BOUNDED_ABOVE as i32;

        /**
        Hint TOGGLED indicates that the data item should be
        considered a Boolean toggle. Data less than or equal to zero should
        be considered `off' or `false,' and data above zero should be
        considered `on' or `true.' TOGGLED may not be used in
        conjunction with any other hint except DEFAULT_0 or
        DEFAULT_1.
         */
        const TOGGLED = ffi::FLUID_HINT_TOGGLED as i32;

        /**
        Hint SAMPLE_RATE indicates that any bounds specified
        should be interpreted as multiples of the sample rate. For
        instance, a frequency range from 0Hz to the Nyquist frequency (half
        the sample rate) could be requested by this hint in conjunction
        with LowerBound = 0 and UpperBound = 0.5. Hosts that support bounds
        at all must support this hint to retain meaning.
         */
        const SAMPLE_RATE = ffi::FLUID_HINT_SAMPLE_RATE as i32;

        /**
        Hint LOGARITHMIC indicates that it is likely that the
        user will find it more intuitive to view values using a logarithmic
        scale. This is particularly useful for frequencies and gains.
         */
        const LOGARITHMIC = ffi::FLUID_HINT_LOGARITHMIC as i32;

        /**
        Hint INTEGER indicates that a user interface would
        probably wish to provide a stepped control taking only integer
        values. Any bounds set should be slightly wider than the actual
        integer range required to avoid floating point rounding errors. For
        instance, the integer set {0,1,2,3} might be described as [-0.1,
        3.1].
         */
        const INTEGER = ffi::FLUID_HINT_INTEGER as i32;

        const FILENAME = ffi::FLUID_HINT_FILENAME as i32;

        const OPTIONLIST = ffi::FLUID_HINT_OPTIONLIST as i32;
    }
}

/**
The single setting of specific type
 */
pub struct Setting<'a, T: ?Sized> {
    handle: *mut ffi::fluid_settings_t,
    name: CString,
    phantom: PhantomData<(&'a (), T)>
}

impl<'a, T> Setting<'a, T>
where
    T: ?Sized,
{
    #[inline]
    fn name_ptr(&self) -> *const raw::c_char {
        self.name.as_ptr() as *const _
    }

    pub fn hints(&self) -> Hints {
        Hints::from_bits_truncate(unsafe { ffi::fluid_settings_get_hints(self.handle, self.name_ptr()) })
    }

    /** Returns whether the setting is changeable in real-time
     */
    pub fn is_realtime(&self) -> bool {
        0 < unsafe { ffi::fluid_settings_is_realtime(self.handle, self.name_ptr()) }
    }
}

impl<'a> Setting<'a, str> {
    /**
    Set the value of a string setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set<S: Into<String>>(&self, value: S) -> bool {
        let mut value = value.into();
        value.push('\0');
        0 < unsafe { ffi::fluid_settings_setstr(self.handle, self.name_ptr(), value.as_ptr() as *const _) }
    }

    /**
    Get the value of a string setting

    Returns `Some("value")` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<&str> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getstr(self.handle, self.name_ptr(), value.as_mut_ptr()) } {
            let value = unsafe { value.assume_init() };
            let value = unsafe { CStr::from_ptr(value) };
            value.to_str().ok()
        } else {
            None
        }
    }

    /**
    Get the default value of a string setting
     */
    pub fn default(&self) -> &str {
        let value = unsafe { ffi::fluid_settings_getstr_default(self.handle, self.name_ptr()) };
        let value = unsafe { CStr::from_ptr(value) };
        value.to_str().unwrap()
    }
}

impl<'a, S> PartialEq<S> for Setting<'a, str>
where
    S: AsRef<str>,
{
    fn eq(&self, other: &S) -> bool {
        let mut other = String::from(other.as_ref());
        other.push('\0');
        0 < unsafe { ffi::fluid_settings_str_equal(self.handle, self.name_ptr(), other.as_ptr() as *mut _) }
    }
}

/**
The range of setting value
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range<T> {
    /// Below limit
    pub min: Option<T>,
    /// Above limit
    pub max: Option<T>,
}

impl<T> Range<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> Self {
        Self { min, max }
    }

    fn new_unsafe(min: MaybeUninit<T>, max: MaybeUninit<T>, hints: Hints) -> Self {
        Self::new(
            if hints.contains(Hints::BOUNDED_BELOW) {
                Some(unsafe { min.assume_init() })
            } else {
                None
            },
            if hints.contains(Hints::BOUNDED_ABOVE) {
                Some(unsafe { max.assume_init() })
            } else {
                None
            },
        )
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        if let Some(value) = &self.min {
            Bound::Included(value)
        } else {
            Bound::Unbounded
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        if let Some(value) = &self.max {
            Bound::Included(value)
        } else {
            Bound::Unbounded
        }
    }
}

impl<'a> Setting<'a, f64> {
    /**
    Set the value of a numeric setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set(&self, value: f64) -> bool {
        0 < unsafe { ffi::fluid_settings_setnum(self.handle, self.name_ptr(), value) }
    }

    /**
    Get the value of a numeric setting

    Returns `Some(value)` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<f64> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getnum(self.handle, self.name_ptr(), value.as_mut_ptr()) } {
            let value = unsafe { value.assume_init() };
            Some(value)
        } else {
            None
        }
    }

    /**
    Get the default value of a numeric setting
     */
    pub fn default(&self) -> f64 {
        unsafe { ffi::fluid_settings_getnum_default(self.handle, self.name_ptr()) }
    }

    /**
    Get the range of values of a numeric setting
     */
    pub fn range(&self) -> Range<f64> {
        let mut min = MaybeUninit::uninit();
        let mut max = MaybeUninit::uninit();

        unsafe { ffi::fluid_settings_getnum_range(self.handle, self.name_ptr(), min.as_mut_ptr(), max.as_mut_ptr()); }

        let hints = self.hints();
        Range::new_unsafe(min, max, hints)
    }
}

impl<'a> Setting<'a, i32> {
    /**
    Set the value of a integer setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set(&self, value: i32) -> bool {
        0 < unsafe { ffi::fluid_settings_setint(self.handle, self.name_ptr(), value) }
    }

    /**
    Get the value of a integer setting

    Returns `Some(value)` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<i32> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getint(self.handle, self.name_ptr(), value.as_mut_ptr()) } {
            let value = unsafe { value.assume_init() };
            Some(value)
        } else {
            None
        }
    }

    /**
    Get the default value of a integer setting
     */
    pub fn default(&self) -> i32 {
        unsafe { ffi::fluid_settings_getint_default(self.handle, self.name_ptr()) }
    }

    /**
    Get the range of values of a integer setting
     */
    pub fn range(&self) -> Range<i32> {
        let mut min = MaybeUninit::uninit();
        let mut max = MaybeUninit::uninit();

        unsafe { ffi::fluid_settings_getint_range(self.handle, self.name_ptr(), min.as_mut_ptr(), max.as_mut_ptr()); }

        let hints = self.hints();
        Range::new_unsafe(min, max, hints)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn settings() {
        let settings = Settings::new().unwrap();

        drop(settings);
    }

    #[test]
    fn num_setting() {
        let settings = Settings::new().unwrap();
        let gain = settings.num("synth.gain").unwrap();

        assert_eq!(gain.default(), 0.2f32 as f64);
        //assert_eq!(gain.range().min, Some(0.0));
        //assert_eq!(gain.range().max, Some(10.0));

        assert_eq!(gain.get(), Some(0.2f32 as f64));
        assert!(gain.set(0.5));
        assert_eq!(gain.get(), Some(0.5));
    }

    #[test]
    fn int_setting() {
        let settings = Settings::new().unwrap();
        let polyphony = settings.int("synth.polyphony").unwrap();

        assert_eq!(polyphony.default(), 256);
        //assert_eq!(polyphony.range().min, Some(1));
        //assert_eq!(polyphony.range().max, Some(65535));

        assert_eq!(polyphony.get(), Some(256));
        assert!(polyphony.set(512));
        assert_eq!(polyphony.get(), Some(512));
    }

    #[test]
    fn str_setting() {
        let settings = Settings::new().unwrap();
        let active = settings.str_("synth.drums-channel.active").unwrap();

        assert_eq!(active.default(), "yes");

        assert_eq!(active.get(), Some("yes"));
        assert!(active.set("no"));
        assert_eq!(active.get(), Some("no"));
    }
}
