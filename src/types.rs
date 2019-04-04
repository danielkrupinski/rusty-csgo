use winapi::ctypes::{c_char, c_int, c_void};

pub type CreateInterfaceFn = unsafe extern fn(name: *const c_char, return_code: *const c_int) -> *const c_void;
