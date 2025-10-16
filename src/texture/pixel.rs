use crate::{structs::Color, texture::ffi::Fade};

#[inline]
pub fn fade(color: Color, alpha: f32) -> Color {
  return unsafe { Fade(color, alpha) };
}
