use std::ffi::CString;

use crate::{
  structs::{RenderTexture2D, Texture2D},
  texture::ffi::{LoadRenderTexture, LoadTexture, UnloadTexture},
};

#[inline]
pub fn load_texture(filename: &str) -> Texture2D {
  let filename = CString::new(filename).expect("Invalid filename");

  return unsafe { LoadTexture(filename.as_ptr()) };
}

#[inline]
pub fn load_render_texture(width: i32, height: i32) -> RenderTexture2D {
  return unsafe { LoadRenderTexture(width, height) };
}

#[inline]
pub fn unload_texture(texture: Texture2D) {
  unsafe { UnloadTexture(texture) };
}
