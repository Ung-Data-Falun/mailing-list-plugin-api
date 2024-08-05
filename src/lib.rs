#![doc = include_str!("../README.md")]

use std::os::raw::c_char;

macro_rules! pluginFn {
    ( $o:ty, $( $x:ty ),* ) => {
         Optional<unsafe extern "C" fn($($x,)*) -> $o>
    };
}

/// A C-Compatible `Option` type
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum Optional<T> {
    Some(T),
    None,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// A Plugin type
pub struct Plugin {
    pub on_start: pluginFn!((), ),
    pub message_handler: pluginFn!((), *const c_char),
}

