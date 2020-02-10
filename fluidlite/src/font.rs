use std::marker::PhantomData;
use crate::{ffi, FontId, Bank, PresetId};

/**
The SoundFont interface
 */
pub trait IsFont {
    fn get_id(&self) -> FontId;
    fn get_name(&self) -> Option<&str>;
    fn get_preset(&self, bank: Bank, num: PresetId) -> Option<PresetRef<'_>>;
}

/**
The SoundFont preset interface
 */
pub trait IsPreset {
    fn get_name(&self) -> Option<&str>;
    fn get_banknum(&self) -> Option<Bank>;
    fn get_num(&self) -> Option<PresetId>;
}

/**
Reference to SoundFont object
 */
#[repr(transparent)]
pub struct FontRef<'a> {
    handle: *mut ffi::fluid_sfont_t,
    phantom: PhantomData<&'a ()>,
}

impl<'a> FontRef<'a> {
    pub(crate) fn from_ptr(handle: *mut ffi::fluid_sfont_t) -> Self {
        Self { handle, phantom: PhantomData }
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::fluid_sfont_t {
        self.handle
    }
}

/**
Reference to Preset object
 */
#[repr(transparent)]
pub struct PresetRef<'a> {
    handle: *mut ffi::fluid_preset_t,
    phantom: PhantomData<&'a ()>,
}

impl<'a> PresetRef<'a> {
    pub(crate) fn from_ptr(handle: *mut ffi::fluid_preset_t) -> Self {
        Self { handle, phantom: PhantomData }
    }
}

mod private {
    use std::ffi::CStr;
    use crate::{ffi, IsFont, FontRef, IsPreset, PresetRef, private::HasHandle, FontId, Bank, PresetId, option_from_ptr};

    impl<X> IsFont for X where X: HasHandle<Handle = ffi::fluid_sfont_t> {
        fn get_id(&self) -> FontId {
            let handle = self.get_handle();
            let font_c = unsafe { &*handle };
            font_c.id
        }

        fn get_name(&self) -> Option<&str> {
            let handle = self.get_handle();
            let font_c = unsafe { &*handle };
            let get_name = font_c.get_name?;
            let name = unsafe { (get_name)(handle) };
            let name = unsafe { CStr::from_ptr(name) };
            name.to_str().ok()
        }

        fn get_preset(&self, bank: Bank, num: PresetId) -> Option<PresetRef<'_>> {
            let handle = self.get_handle();
            let font_c = unsafe { &*handle };
            let get_preset = font_c.get_preset?;
            option_from_ptr(unsafe { (get_preset)(handle, bank, num) })
                .map(PresetRef::from_ptr)
        }
    }

    impl<'a> HasHandle for FontRef<'a> {
        type Handle = ffi::fluid_sfont_t;

        fn get_handle(&self) -> *mut Self::Handle {
            self.handle
        }
    }

    impl<X> IsPreset for X where X: HasHandle<Handle = ffi::fluid_preset_t> {
        fn get_name(&self) -> Option<&str> {
            let handle = self.get_handle();
            let font_c = unsafe { &*handle };
            let get_name = font_c.get_name?;
            let name = unsafe { (get_name)(handle) };
            let name = unsafe { CStr::from_ptr(name) };
            name.to_str().ok()
        }

        fn get_banknum(&self) -> Option<Bank> {
            let handle = self.get_handle();
            let preset_c = unsafe { &*handle };
            let get_banknum = preset_c.get_banknum?;
            let num = unsafe { (get_banknum)(handle) };
            if num < 0 {
                None
            } else {
                Some(num as _)
            }
        }

        fn get_num(&self) -> Option<PresetId> {
            let handle = self.get_handle();
            let preset_c = unsafe { &*handle };
            let get_num = preset_c.get_num?;
            let num = unsafe { (get_num)(handle) };
            if num < 0 {
                None
            } else {
                Some(num as _)
            }
        }
    }

    impl<'a> HasHandle for PresetRef<'a> {
        type Handle = ffi::fluid_preset_t;

        fn get_handle(&self) -> *mut Self::Handle {
            self.handle
        }
    }
}
