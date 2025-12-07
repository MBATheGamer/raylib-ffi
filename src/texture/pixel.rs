use crate::{
  structs::Color,
  texture::ffi::{ColorAlpha, Fade, GetColor},
};

#[inline]
pub fn fade(color: Color, alpha: f32) -> Color {
  return unsafe { Fade(color, alpha) };
}

#[inline]
pub fn color_alpha(color: Color, alpha: f32) -> Color {
  return unsafe { ColorAlpha(color, alpha) };
}

#[inline]
pub fn get_color(hex_value: u32) -> Color {
  return unsafe { GetColor(hex_value) };
}
