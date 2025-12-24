use crate::{
  structs::Color,
  texture::ffi::{ColorAlpha, ColorLerp, Fade, GetColor},
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
pub fn color_lerp(color1: Color, color2: Color, factor: f32) -> Color {
  return unsafe { ColorLerp(color1, color2, factor) };
}

#[inline]
pub fn get_color(hex_value: u32) -> Color {
  return unsafe { GetColor(hex_value) };
}
