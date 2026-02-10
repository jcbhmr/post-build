use std::ffi::{CStr, CString, c_char};

pub extern "C" fn greet(name: *const c_char) -> *mut c_char {
    let name = unsafe { CStr::from_ptr(name) };
    let name = name.to_str().expect("name should be valid UTF-8");
    let greeting = format!("Hi there, {}!", name);
    let greeting = CString::new(greeting).expect("greeting should not contain NUL");
    greeting.into_raw()
}
