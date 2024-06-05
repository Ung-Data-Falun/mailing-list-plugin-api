macro_rules! pluginFn {
    ( $o:ty, $( $x:ty ),* ) => {
         Option<*mut extern "C" fn($($x,)*) -> $o>
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Plugin {
    pub message_handler: pluginFn!((), String),
}

