use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn free_pointer(ptr: *mut c_void) {
  if !ptr.is_null() {
    unsafe {
      drop(Box::from_raw(ptr as *mut usize));
    }
  }
}
