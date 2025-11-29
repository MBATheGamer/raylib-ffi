use crate::{
  structs::{Color, Image},
  texture::ffi::{GenImageGradientLinear, GenImageGradientRadial},
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

#[inline]
pub fn gen_image_gradient_radial(
  width: i32,
  height: i32,
  density: f32,
  inner: Color,
  outer: Color,
) -> Image {
  return unsafe { GenImageGradientRadial(width, height, density, inner, outer) };
}
