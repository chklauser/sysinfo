// Take a look at the license at the top of the repository in the LICENSE file.

use libc::c_char;
use std::ffi::{CStr, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

/// Something that _can_ be a pointer to a C string. Unlike `CStr`, the interface of this trait
/// returns `Option`s and can thus check for null pointers.
///
/// The trait generally cannot check whether the thing it points to is actually a C string.
/// It's only safe to use with C strings.
pub(crate) trait CStrPtr {
    /// # Safety
    ///  1. `self` must be a zero-terminated C string.
    unsafe fn cstr_to_str(&self) -> Option<&str> {
        self.as_cstr()?.to_str().ok()
    }

    /// # Safety
    ///  1. `self` must be a zero-terminated C string.
    unsafe fn cstr_to_string(&self) -> Option<String> {
        self.cstr_to_str().map(|s| s.to_owned())
    }

    /// # Safety
    ///  1. `self` must be a zero-terminated C string.
    unsafe fn cstr_to_os_string(&self) -> Option<OsString> {
        Some(OsStr::from_bytes(self.as_cstr()?.to_bytes()).to_os_string())
    }

    /// # Safety
    ///  1. `self` must be a zero-terminated C string.
    unsafe fn as_cstr(&self) -> Option<&CStr>;
}

impl CStrPtr for *const c_char {
    unsafe fn as_cstr(&self) -> Option<&CStr> {
        if self.is_null() {
            None
        } else {
            Some(CStr::from_ptr(*self))
        }
    }
}

impl CStrPtr for *mut c_char {
    unsafe fn as_cstr(&self) -> Option<&CStr> {
        if self.is_null() {
            None
        } else {
            Some(CStr::from_ptr(*self))
        }
    }
}

impl CStrPtr for [c_char] {
    unsafe fn as_cstr(&self) -> Option<&CStr> {
        let bytes_u8 = &*(self as *const [c_char] as *const [u8]);
        CStr::from_bytes_with_nul(bytes_u8).ok()
    }
}
