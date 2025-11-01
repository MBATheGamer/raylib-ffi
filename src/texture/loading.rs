use std::ffi::CString;

use crate::{
  structs::{RenderTexture, Texture},
  texture::ffi::{LoadRenderTexture, LoadTexture, UnloadRenderTexture, UnloadTexture},
};

#[inline]
pub fn load_texture(filename: &str) -> Texture {
  let filename = CString::new(filename).expect("Invalid filename");

  return unsafe { LoadTexture(filename.as_ptr()) };
}

#[inline]
pub fn load_render_texture(width: i32, height: i32) -> RenderTexture {
  return unsafe { LoadRenderTexture(width, height) };
}

#[inline]
pub fn unload_texture(texture: Texture) {
  unsafe { UnloadTexture(texture) };
}

#[inline]
pub fn unload_render_texture(target: RenderTexture) {
  unsafe { UnloadRenderTexture(target) };
}
