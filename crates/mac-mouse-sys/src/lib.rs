//! Tiny wrapper for mouse-related functions in macOS `IOKit/hidsystem`
//!
//! Definition of terms:
//!
//! `pointer resolution` - the resolution of pointer movement space, the higher the value, the slower the pointer movement speed
//!
//! Special values:
//! 
//! - `-1`: null
//!
//! - `5 * 65536`: Min
//!
//! - `1995 * 65536` Max
//!
//! `mouse acceleration` - the acceleration of the pointer movement, the higher the value, the faster the pointer speed will increase, `0` means no acceleration
//!
//! Special values:
//!
//! - `0`: No acceleration
//!
//! - `45056`: macOS default

#![warn(trivial_casts, trivial_numeric_casts, missing_docs)]
#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

mod sys {
    use libc::c_int;
    #[link(name = "mouse")]
    extern "C" {
        pub fn getPointerResolution() -> c_int;
        pub fn getMouseAcceleration() -> c_int;
        pub fn setPointerResolution(res: c_int) -> c_int;
        pub fn setMouseAcceleration(acc: c_int) -> c_int;
    }
}

/// Get the pointer resolution
pub fn get_pointer_resolution() -> Result<i32, String> {
    let res = unsafe { sys::getPointerResolution() };
    Ok(res)
}

/// Get the mouse acceleration
pub fn get_mouse_acceleration() -> Result<i32, String> {
    let acc = unsafe { sys::getMouseAcceleration() };
    if acc == -1 {
        Err("Failed to get mouse acceleration".to_string())
    } else {
        Ok(acc)
    }
}

/// Set the pointer resolution
pub fn set_pointer_resolution(res: i32) -> Result<(), String> {
    let ret = unsafe { sys::setPointerResolution(res) };
    if ret != 0 {
        Err("Failed to set pointer resolution".to_string())
    } else {
        Ok(())
    }
}

/// Set the mouse acceleration
pub fn set_mouse_acceleration(acc: i32) -> Result<(), String> {
    let ret = unsafe { sys::setMouseAcceleration(acc) };
    if ret != 0 {
        Err("Failed to set mouse acceleration".to_string())
    } else {
        Ok(())
    }
}
