use crate::{
  structs::{Color, Image},
  texture::ffi::{
    GenImageChecked, GenImageGradientLinear, GenImageGradientRadial, GenImageGradientSquare,
  },
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

#[inline]
pub fn gen_image_gradient_square(
  width: i32,
  height: i32,
  density: f32,
  inner: Color,
  outer: Color,
) -> Image {
  return unsafe { GenImageGradientSquare(width, height, density, inner, outer) };
}

#[inline]
pub fn gen_image_checked(
  width: i32,
  height: i32,
  checks_x: i32,
  checks_y: i32,
  col1: Color,
  col2: Color,
) -> Image {
  return unsafe { GenImageChecked(width, height, checks_x, checks_y, col1, col2) };
}
