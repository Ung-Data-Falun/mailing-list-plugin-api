use std::os::raw::c_char;

macro_rules! pluginFn {
    ( $o:ty, $( $x:ty ),* ) => {
         Optional<unsafe extern "C" fn($($x,)*) -> $o>
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum Optional<T> {
    Some(T),
    None,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Plugin {
    pub message_handler: pluginFn!((), *const c_char),
}

