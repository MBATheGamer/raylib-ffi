use std::ffi::CString;

use crate::{
  structs::{Font, Vector2},
  text::ffi::{MeasureText, MeasureTextEx},
};

#[inline]
pub fn measure_text(text: &str, font_size: i32) -> i32 {
  let text = CString::new(text).expect("You must set a text in this function");

  return unsafe { MeasureText(text.as_ptr(), font_size) };
}

#[inline]
pub fn measure_text_ex(font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
  let text = CString::new(text).expect("You must set a text in this function");

  return unsafe { MeasureTextEx(font, text.as_ptr(), font_size, spacing) };
}
