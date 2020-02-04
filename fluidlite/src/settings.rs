use std::{
    marker::PhantomData,
    os::raw,
    mem::MaybeUninit,
    ffi::CStr,
    ops::{Bound, RangeBounds},
};
use bitflags::bitflags;
use crate::ffi;

/**
The generic settings object
 */
#[repr(transparent)]
pub struct Settings {
    handle: *mut ffi::fluid_settings_t,
}

impl Default for Settings {
    fn default() -> Self {
        let handle = unsafe { ffi::new_fluid_settings() };

        assert!(!handle.is_null());

        Self { handle }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        unsafe { ffi::delete_fluid_settings(self.handle); }
    }
}

impl Settings {
    pub(crate) fn ptr(&self) -> *mut ffi::fluid_settings_t {
        self.handle
    }

    /*pub(crate) fn into_ptr(self) -> *mut ffi::fluid_settings_t {
        self.handle
    }*/

    pub fn pick<'a: 's, 's, S, T>(&'a self, name: S) -> Option<Setting<'s, T>>
    where
        S: Into<String>,
        T: IsSetting + ?Sized,
    {
        let mut name = name.into();
        name.push('\0');
        T::new(self, name)
    }

    pub fn str_<'a: 's, 's, S>(&'a self, name: S) -> Option<Setting<'s, str>>
    where
        S: Into<String>,
    {
        self.pick(name)
    }

    pub fn num<'a: 's, 's, S>(&'a self, name: S) -> Option<Setting<'s, f64>>
    where
        S: Into<String>,
    {
        self.pick(name)
    }

    pub fn int<'a: 's, 's, S>(&'a self, name: S) -> Option<Setting<'s, i32>>
    where
        S: Into<String>,
    {
        self.pick(name)
    }
}

pub trait IsSetting {
    const TYPE: ffi::fluid_types_enum;

    fn new<'a: 's, 's>(settings: &'a Settings, name: String) -> Option<Setting<'s, Self>> {
        if Self::TYPE == unsafe { ffi::fluid_settings_get_type(settings.handle, name.as_ptr() as *const _) } {
            Some(Setting { settings, name, phantom: PhantomData })
        } else {
            None
        }
    }
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
pub struct Setting<'s, T: ?Sized> {
    settings: &'s Settings,
    name: String,
    phantom: PhantomData<T>
}

impl<'s, T> Setting<'s, T>
where
    T: ?Sized,
{
    #[inline]
    fn settings_ptr(&self) -> *mut ffi::fluid_settings_t {
        self.settings.handle
    }

    #[inline]
    fn name_ptr(&self) -> *const raw::c_char {
        self.name.as_ptr() as *const _
    }

    pub fn hints(&self) -> Hints {
        Hints::from_bits_truncate(unsafe { ffi::fluid_settings_get_hints(self.settings_ptr(), self.name_ptr()) })
    }

    /** Returns whether the setting is changeable in real-time
     */
    pub fn is_realtime(&self) -> bool {
        0 < unsafe { ffi::fluid_settings_is_realtime(self.settings_ptr(), self.name_ptr()) }
    }
}

impl<'s> Setting<'s, str> {
    /**
    Set the value of a string setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set<S: Into<String>>(&self, value: S) -> bool {
        let mut value = value.into();
        value.push('\0');
        0 < unsafe { ffi::fluid_settings_setstr(self.settings_ptr(), self.name_ptr(), value.as_ptr() as *const _) }
    }

    /**
    Get the value of a string setting

    Returns `Some("value")` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<&str> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getstr(self.settings_ptr(), self.name_ptr(), value.as_mut_ptr()) } {
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
        let value = unsafe { ffi::fluid_settings_getstr_default(self.settings_ptr(), self.name_ptr()) };
        let value = unsafe { CStr::from_ptr(value) };
        value.to_str().unwrap()
    }
}

impl<'s, S> PartialEq<S> for Setting<'s, str>
where
    S: AsRef<str>,
{
    fn eq(&self, other: &S) -> bool {
        let mut other = String::from(other.as_ref());
        other.push('\0');
        0 < unsafe { ffi::fluid_settings_str_equal(self.settings_ptr(), self.name_ptr(), other.as_ptr() as *mut _) }
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

impl<'s> Setting<'s, f64> {
    /**
    Set the value of a numeric setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set(&self, value: f64) -> bool {
        0 < unsafe { ffi::fluid_settings_setnum(self.settings_ptr(), self.name_ptr(), value) }
    }

    /**
    Get the value of a numeric setting

    Returns `Some(value)` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<f64> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getnum(self.settings_ptr(), self.name_ptr(), value.as_mut_ptr()) } {
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
        unsafe { ffi::fluid_settings_getnum_default(self.settings_ptr(), self.name_ptr()) }
    }

    /**
    Get the range of values of a numeric setting
     */
    pub fn range(&self) -> Range<f64> {
        let mut min = MaybeUninit::uninit();
        let mut max = MaybeUninit::uninit();

        unsafe { ffi::fluid_settings_getnum_range(self.settings_ptr(), self.name_ptr(), min.as_mut_ptr(), max.as_mut_ptr()); }

        let hints = self.hints();
        Range::new_unsafe(min, max, hints)
    }
}

impl<'s> Setting<'s, i32> {
    /**
    Set the value of a integer setting

    Returns `true` if the value has been set, `false` otherwise
     */
    pub fn set(&self, value: i32) -> bool {
        0 < unsafe { ffi::fluid_settings_setint(self.settings_ptr(), self.name_ptr(), value) }
    }

    /**
    Get the value of a integer setting

    Returns `Some(value)` if the value exists, `None` otherwise
     */
    pub fn get(&self) -> Option<i32> {
        let mut value = MaybeUninit::uninit();

        if 0 < unsafe { ffi::fluid_settings_getint(self.settings_ptr(), self.name_ptr(), value.as_mut_ptr()) } {
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
        unsafe { ffi::fluid_settings_getint_default(self.settings_ptr(), self.name_ptr()) }
    }

    /**
    Get the range of values of a integer setting
     */
    pub fn range(&self) -> Range<i32> {
        let mut min = MaybeUninit::uninit();
        let mut max = MaybeUninit::uninit();

        unsafe { ffi::fluid_settings_getint_range(self.settings_ptr(), self.name_ptr(), min.as_mut_ptr(), max.as_mut_ptr()); }

        let hints = self.hints();
        Range::new_unsafe(min, max, hints)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn settings() {
        let settings = Settings::default();

        drop(settings);
    }

    #[test]
    fn num_setting() {
        let settings = Settings::default();
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
        let settings = Settings::default();
        let polyphony = settings.int("synth.polyphony").unwrap();

        assert_eq!(polyphony.default(), 256);
        //assert_eq!(polyphony.range().min, Some(1));
        //assert_eq!(polyphony.range().max, Some(65535));

        assert_eq!(polyphony.get(), Some(256));
        assert!(polyphony.set(512));
        assert_eq!(polyphony.get(), Some(512));
    }

    /*#[test]
    fn str_setting() {
        let settings = Settings::default();
        let bank = settings.str_("synth.midi-bank-select").unwrap();

        assert_eq!(bank.default(), "gs");
    }*/
}
