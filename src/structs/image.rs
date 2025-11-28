use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Image {
  pub data: *mut c_void, // Image raw data
  pub width: i32,        // Image base width
  pub height: i32,       // Image base height
  pub mipmaps: i32,      // Mipmap levels, 1 by default
  pub format: i32,       // Data format (PixelFormat type)
}
