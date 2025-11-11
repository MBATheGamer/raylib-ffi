use crate::{
  structs::Image,
  texture::ffi::{IsImageValid, UnloadImage},
};

#[inline]
pub fn is_image_valid(image: Image) -> bool {
  return unsafe { IsImageValid(image) };
}

#[inline]
pub fn unload_image(image: Image) {
  unsafe { UnloadImage(image) };
}
