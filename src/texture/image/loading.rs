use std::ffi::CString;

use crate::{
  enums::PixelFormat,
  structs::{Image, Texture},
  texture::ffi::{IsImageValid, LoadImage, LoadImageFromTexture, LoadImageRaw, UnloadImage},
};

#[inline]
pub fn load_image(file_name: &str) -> Image {
  let file_name = CString::new(file_name).expect("Expecting image path!");

  return unsafe { LoadImage(file_name.as_ptr()) };
}

#[inline]
pub fn load_image_raw(
  file_name: &str,
  width: i32,
  height: i32,
  format: PixelFormat,
  header_size: i32,
) -> Image {
  let file_name = CString::new(file_name).expect("Expecting image path!");

  return unsafe {
    LoadImageRaw(
      file_name.as_ptr(),
      width,
      height,
      format as i32,
      header_size,
    )
  };
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
