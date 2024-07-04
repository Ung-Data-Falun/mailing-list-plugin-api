# mlpa

mlpa is an API library for writing plugins for [mailing-list](https://github.com/Ung-Data-Falun/mailing-list)

## Examples

A plugin that says hello:
```rust
use std::ffi::CStr;
use std::os::raw::c_char;
use mlpa::Plugin;

#[no_mangle]
pub extern "C" fn get_plugin() -> Plugin {
    use mlpa::Optional::Some;
    Plugin {
        message_handler: Some(message_handler),
    }
}

extern "C" fn message_handler(message: *const c_char) {
    let message = unsafe { CStr::from_ptr(message) };
    let message = String::from_utf8_lossy(message.to_bytes()).to_string();

    println!("Hello from plugin!");
    println!("Message: {message}");
}

```

to build and use in mailing-list:

1. Add to `Cargo.toml`
```toml
[lib]
crate-type = ["dylib"]
```
2. Build your plugin
4. Add to `daemon.toml`
```toml
plugins = [
    "[your plugin]"
]
```


