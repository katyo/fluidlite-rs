use crate::ffi;
use std::{
    ffi::{CStr, CString},
    fmt::{Display, Formatter, Result as FmtResult},
    os::raw,
    ptr::null_mut,
};

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
Closure logger wrapper

```
# use fluidlite_lib as _;
use fluidlite::{Log, LogLevel, FnLogger};

Log::set(&LogLevel::DEBUG,
  FnLogger::from(|level, message: &str| {
    eprintln!("[{}]: {}", level, message);
  })
);
```
 */
pub struct FnLogger<F>(F);

impl<F> FnLogger<F>
where
    F: FnMut(LogLevel, &str),
{
    pub fn new(func: F) -> Self {
        Self(func)
    }
}

impl<F> From<F> for FnLogger<F>
where
    F: FnMut(LogLevel, &str),
{
    fn from(func: F) -> Self {
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
pub struct Log {
    levels: Vec<LogLevel>,
    #[used]
    logger: Box<dyn Logger>,
}

impl Drop for Log {
    fn drop(&mut self) {
        for level in &self.levels {
            unsafe {
                ffi::fluid_set_log_function(
                    *level as i32,
                    Some(ffi::fluid_default_log_function),
                    null_mut(),
                );
            }
        }
    }
}

impl Log {
    /// Create and install logger
    fn new<I, T>(levels: I, logger: T) -> Self
    where
        T: Logger + 'static,
        I: AsRef<[LogLevel]>,
    {
        let levels = Vec::from(levels.as_ref());
        let logger = Box::new(logger);

        for level in &levels {
            unsafe {
                ffi::fluid_set_log_function(
                    *level as i32,
                    Some(handler::<T>),
                    logger.as_ref() as *const _ as *mut _,
                );
            }
        }

        Log { levels, logger }
    }
}

fn with_global_logger(func: impl FnOnce(&mut Option<Log>)) {
    use std::sync::{Arc, Mutex, Once};

    static ONCE: Once = Once::new();
    static mut LOG: *mut Arc<Mutex<Option<Log>>> = null_mut();

    ONCE.call_once(|| unsafe {
        LOG = Box::into_raw(Box::new(Arc::new(Mutex::new(None))));
    });

    let log = (unsafe { &*LOG }).clone();
    let mut log = log.lock().unwrap();

    func(&mut log);
}

impl Log {
    /// Set logger
    pub fn set<I, T>(levels: I, logger: T)
    where
        T: Logger + 'static,
        I: AsRef<[LogLevel]>,
    {
        with_global_logger(|global_logger| {
            if global_logger.is_some() {
                *global_logger = None;
            }
            *global_logger = Some(Log::new(levels, logger));
        });
    }

    /// Reset logger
    pub fn reset() {
        with_global_logger(|global_logger| {
            *global_logger = None;
        });
    }
}

impl Log {
    pub fn default_log(level: LogLevel, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe {
            ffi::fluid_default_log_function(level as _, message.as_ptr() as *mut _, null_mut());
        }
    }
}

#[cfg(feature = "log")]
pub use self::log::LogLogger;

#[cfg(feature = "log")]
mod log {
    use super::{LogLevel, Logger};
    use log::{log, Level};

    /**
    Logger implementation backed by [log](https://crates.io/crates/log) crate.

    ```
    # use fluidlite_lib as _;
    use fluidlite::{Log, LogLevel, LogLogger};

    Log::set(&LogLevel::DEBUG, LogLogger::default());
    ```
     */
    pub struct LogLogger<S> {
        target: S,
    }

    impl Default for LogLogger<&'static str> {
        fn default() -> Self {
            Self::new("fluidlite")
        }
    }

    impl<S> LogLogger<S> {
        pub fn new(target: S) -> Self {
            Self { target }
        }
    }

    impl<S: AsRef<str>> Logger for LogLogger<S> {
        fn log(&mut self, level: LogLevel, message: &str) {
            log!(target: self.target.as_ref(), level.into(), "{}", message);
        }
    }

    impl Into<Level> for LogLevel {
        fn into(self) -> Level {
            match self {
                LogLevel::Panic => Level::Error,
                LogLevel::Error => Level::Error,
                LogLevel::Warning => Level::Warn,
                LogLevel::Info => Level::Info,
                LogLevel::Debug => Level::Debug,
            }
        }
    }
}

extern "C" fn handler<T>(level: raw::c_int, message: *mut raw::c_char, data: *mut raw::c_void)
where
    T: Logger,
{
    assert!(!data.is_null());

    let logger = unsafe { &mut *(data as *mut T) };
    let level = LogLevel::from_ffi(level).unwrap();
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    logger.log(level, message);
}
