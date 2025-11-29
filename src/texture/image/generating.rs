use crate::{
  structs::{Color, Image},
  texture::ffi::GenImageGradientLinear,
};

#[inline]
pub fn gen_image_gradient_linear(
  width: i32,
  height: i32,
  direction: i32,
  start: Color,
  end: Color,
) -> Image {
  return unsafe { GenImageGradientLinear(width, height, direction, start, end) };
}
