use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Image {
  data: *mut c_void, // Image raw data
  width: i32,        // Image base width
  height: i32,       // Image base height
  mipmaps: i32,      // Mipmap levels, 1 by default
  format: i32,       // Data format (PixelFormat type)
}
