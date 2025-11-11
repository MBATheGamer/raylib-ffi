use crate::{structs::Image, texture::ffi::IsImageValid};

#[inline]
pub fn is_image_valid(image: Image) -> bool {
  return unsafe { IsImageValid(image) };
}
