use std::ffi::CString;

use crate::{
  structs::{Color, Font, Vector2},
  text::ffi::{DrawFPS, DrawText, DrawTextEx},
};

#[inline]
pub fn draw_fps(pos_x: i32, pos_y: i32) {
  unsafe { DrawFPS(pos_x, pos_y) };
}

#[inline]
pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
  let text = CString::new(text).expect("You must set a text in this function");

  unsafe {
    DrawText(text.as_ptr(), pos_x, pos_y, font_size, color);
  }
}

#[inline]
pub fn draw_text_ex(
  font: Font,
  text: &str,
  position: Vector2,
  font_size: f32,
  spacing: f32,
  tint: Color,
) {
  let text = CString::new(text).expect("You must set a text in this function");

  unsafe { DrawTextEx(font, text.as_ptr(), position, font_size, spacing, tint) };
}
