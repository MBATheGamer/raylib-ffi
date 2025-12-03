use std::ffi::CString;

use crate::{
  structs::{Image, Texture},
  texture::ffi::{IsImageValid, LoadImage, LoadImageFromTexture, UnloadImage},
};

#[inline]
pub fn load_image(file_name: &str) -> Image {
  let file_name = CString::new(file_name).expect("Image path!");

  return unsafe { LoadImage(file_name.as_ptr()) };
}

#[inline]
pub fn load_image_from_texture(texture: Texture) -> Image {
  return unsafe { LoadImageFromTexture(texture) };
}

#[inline]
pub fn is_image_valid(image: Image) -> bool {
  return unsafe { IsImageValid(image) };
}

#[inline]
pub fn unload_image(image: Image) {
  unsafe { UnloadImage(image) };
}
