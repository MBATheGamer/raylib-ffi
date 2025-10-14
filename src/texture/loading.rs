use std::ffi::CString;

use crate::{
  structs::Texture2D,
  texture::ffi::{LoadTexture, UnloadTexture},
};

#[inline]
pub fn load_texture(filename: &str) -> Texture2D {
  let filename = CString::new(filename).expect("Invalid filename");

  return unsafe { LoadTexture(filename.as_ptr()) };
}

#[inline]
pub fn unload_texture(texture: Texture2D) {
  unsafe { UnloadTexture(texture) };
}
