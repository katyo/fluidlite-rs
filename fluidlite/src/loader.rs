use std::{
    mem::transmute,
};
use crate::{Result, result_from_ptr, ffi};

/**
The SoundFont loader object
 */
#[repr(transparent)]
pub struct Loader {
    handle: *mut ffi::fluid_sfloader_t,
}

impl Loader {
    /**
    Create default SoundFont loader
     */
    pub fn new_default() -> Result<Self> {
        result_from_ptr(unsafe { ffi::new_fluid_defsfloader() })
            .map(|handle| Self { handle })
    }

    pub(crate) fn into_ptr(self) -> *mut ffi::fluid_sfloader_t {
        unsafe { transmute(self) }
    }

    /*pub fn set_file_api(&self, fileapi: FileApi) {

    }*/
}

impl Drop for Loader {
    fn drop(&mut self) {
        unsafe { ffi::delete_fluid_defsfloader(self.handle); }
    }
}

/*
/**
File API object for loader
 */
#[repr(transparent)]
pub struct FileApi {
    fileapi:
}
*/
