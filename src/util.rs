use imgui::sys::igMemFree;
use std::ffi::CStr;
use std::os::raw::c_void;

/// Safe wrapper for converting C strings to Rust strings
pub unsafe fn ptr_into_string(ptr: *mut std::os::raw::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        let result = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        igMemFree(ptr as *mut c_void); // Free the C string
        result
    }
}

/// Safe wrapper for converting C strings to Rust strings without freeing
pub unsafe fn ptr_clone_to_string(ptr: *const std::os::raw::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
