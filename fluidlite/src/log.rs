use std::{
    ffi::CStr,
    os::raw,
    ptr::null_mut,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Deref, DerefMut},
};
use crate::ffi;

/**
 * Logging level
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum LogLevel {
    /**
     * The synth can't function correctly any more
     */
    Panic = ffi::fluid_log_level_FLUID_PANIC,

    /**
     * Serious error occurred
     */
    Error = ffi::fluid_log_level_FLUID_ERR,

    /**
     * Warning
     */
    Warning = ffi::fluid_log_level_FLUID_WARN,

    /**
     * Verbose informational messages
     */
    Info = ffi::fluid_log_level_FLUID_INFO,

    /**
     * Debugging messages
     */
    Debug = ffi::fluid_log_level_FLUID_DBG,
}

impl LogLevel {
    fn from_ffi(level: i32) -> Option<Self> {
        use self::LogLevel::*;
        Some(match level as u32 {
            ffi::fluid_log_level_FLUID_PANIC => Panic,
            ffi::fluid_log_level_FLUID_ERR => Error,
            ffi::fluid_log_level_FLUID_WARN => Warning,
            ffi::fluid_log_level_FLUID_INFO => Info,
            ffi::fluid_log_level_FLUID_DBG => Debug,
            _ => return None,
        })
    }

    /// All log levels
    pub const DEBUG: [LogLevel; 5] = {
        use self::LogLevel::*;
        [Panic, Error, Warning, Info, Debug]
    };

    /// All log levels excluding debug
    pub const INFO: [LogLevel; 4] = {
        use self::LogLevel::*;
        [Panic, Error, Warning, Info]
    };

    /// Warning, error and panic log levels
    pub const WARNING: [LogLevel; 3] = {
        use self::LogLevel::*;
        [Panic, Error, Warning]
    };

    /// Error and panic log levels only
    pub const ERROR: [LogLevel; 2] = {
        use self::LogLevel::*;
        [Panic, Error]
    };

    /// Panic log level only
    pub const PANIC: [LogLevel; 1] = {
        use self::LogLevel::*;
        [Panic]
    };
}

impl AsRef<str> for LogLevel {
    fn as_ref(&self) -> &str {
        use self::LogLevel::*;
        match self {
            Panic => "PANIC",
            Error => "ERROR",
            Warning => "WARNING",
            Info => "INFO",
            Debug => "DEBUG",
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.as_ref().fmt(f)
    }
}

/**
 * Log output handler
 */
pub trait Logger {
    /**
     * Log message handling method
     */
    fn log(&mut self, level: LogLevel, message: &str);
}

/**
 * Closure logger wrapper
 */
pub struct FnLogger<F>(F);

impl<F: FnMut(LogLevel, &str)> FnLogger<F> {
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F: FnMut(LogLevel, &str)> Logger for FnLogger<F> {
    fn log(&mut self, level: LogLevel, message: &str) {
        (self.0)(level, message);
    }
}

/**
 * Logging
 *
 * You can use own logger to handle library messages.
 * Only one logger supported at a time.
 * You should keep logger from dropping while it used.
 */
pub struct Log<T> {
    levels: Vec<LogLevel>,
    logger: Box<T>,
}

impl<T> Drop for Log<T> {
    fn drop(&mut self) {
        for level in &self.levels {
            unsafe { ffi::fluid_set_log_function(
                *level as i32,
                Some(ffi::fluid_default_log_function),
                null_mut(),
            ); }
        }
    }
}

impl<T> Deref for Log<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.logger
    }
}

impl<T> DerefMut for Log<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logger
    }
}

impl<T: Logger> Log<T> {
    pub fn new<I: IntoIterator<Item = LogLevel>>(levels: I, logger: T) -> Self {
        let levels: Vec<_> = levels.into_iter().collect();
        let logger = Box::new(logger);

        for level in &levels {
            unsafe { ffi::fluid_set_log_function(
                *level as i32,
                Some(handler::<T>),
                logger.as_ref() as *const _ as *mut _,
            ); }
        }

        Log { levels, logger }
    }
}

impl<F: FnMut(LogLevel, &str)> Log<FnLogger<F>> {
    pub fn new_fn<I: IntoIterator<Item = LogLevel>>(levels: I, func: F) -> Self {
        Log::new(levels, FnLogger::new(func))
    }
}

extern "C" fn handler<T>(
        level: raw::c_int,
        message: *mut raw::c_char,
        data: *mut raw::c_void,
)
where
    T: Logger,
{
    assert!(!data.is_null());

    let logger = unsafe { &mut *(data as *mut T) };
    let level = LogLevel::from_ffi(level).unwrap();
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    logger.log(level, message);
}

pub fn default_log(level: i32, message: &str) {
    let mut message = String::from(message);
    message.push('\0');
    unsafe { ffi::fluid_default_log_function(level, message.as_mut_ptr() as *mut _, null_mut()); }
}
