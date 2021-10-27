#![allow(unused)]

use crate::console::print;
use core::cell::RefCell;
use core::fmt;
use lazy_static::*;

pub struct LogLevel {
    level: u8,
    name: &'static str,
    csi_code: u8,
}

pub const ERROR: LogLevel = LogLevel {
    level: 1,
    name: "ERROR",
    csi_code: 31,
};
pub const WARN: LogLevel = LogLevel {
    level: 2,
    name: "WARN",
    csi_code: 93,
};
pub const INFO: LogLevel = LogLevel {
    level: 3,
    name: "INFO",
    csi_code: 34,
};
pub const DEBUG: LogLevel = LogLevel {
    level: 4,
    name: "DEBUG",
    csi_code: 32,
};
pub const TRACE: LogLevel = LogLevel {
    level: 5,
    name: "TRACE",
    csi_code: 90,
};

struct DefaultLogLevel {
    default_level: RefCell<u8>,
}
unsafe impl Sync for DefaultLogLevel {}

lazy_static! {
    static ref LOG_LEVEL: DefaultLogLevel = DefaultLogLevel {
        default_level: RefCell::new(DEBUG.level),
    };
}

pub fn log_print(level: LogLevel, args: fmt::Arguments) {
    if level.level <= *LOG_LEVEL.default_level.borrow() {
        print(format_args!(
            "\x1b[{}m[{}] {}\n\x1b[0m",
            level.csi_code, level.name, args
        ));
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::log_print($crate::log::TRACE, format_args!($fmt $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::log_print($crate::log::DEBUG, format_args!($fmt $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::log_print($crate::log::INFO, format_args!($fmt $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::log_print($crate::log::WARN, format_args!($fmt $(, $($arg)+)?));
    }
}
#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::log_print($crate::log::ERROR, format_args!($fmt $(, $($arg)+)?));
    }
}
