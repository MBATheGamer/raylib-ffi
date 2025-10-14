use std::ffi::CString;

use crate::{structs::Texture2D, texture::ffi::LoadTexture};

#[inline]
pub fn load_texture(filename: &str) -> Texture2D {
  let filename = CString::new(filename).expect("Invalid filename");

  return unsafe { LoadTexture(filename.as_ptr()) };
}
