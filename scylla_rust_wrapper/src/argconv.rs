use std::ffi::{c_char, CStr};

use crate::types::size_t;

pub unsafe fn ptr_to_ref<T>(ptr: *const T) -> &'static T {
  ptr.as_ref().unwrap()
}

pub unsafe fn ptr_to_ref_mut<T>(ptr: *mut T) -> &'static mut T {
  ptr.as_mut().unwrap()
}

pub unsafe fn cstr_to_str(ptr: *const c_char) -> Option<&'static str> {
  CStr::from_ptr(ptr).to_str().ok()
}

pub unsafe fn ptr_to_str_n(ptr: *const c_char, size: size_t) -> Option<&'static str> {
  if ptr.is_null() {
    return None;
  }
  std::str::from_utf8(std::slice::from_raw_parts(ptr as *const u8, size as usize)).ok()
}
