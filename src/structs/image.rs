use std::ffi::c_void;

use crate::enums::PixelFormat;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Image {
  pub data: *mut c_void,   // Image raw data
  pub width: i32,          // Image base width
  pub height: i32,         // Image base height
  pub mipmaps: i32,        // Mipmap levels, 1 by default
  pub format: PixelFormat, // Data format (PixelFormat type)
}
