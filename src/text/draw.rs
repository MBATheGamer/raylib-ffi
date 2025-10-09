use std::ffi::CString;

use crate::{structs::Color, text::ffi::DrawText};

#[inline]
pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
  let text = CString::new(text).expect("You must set a text in this function");

  unsafe {
    DrawText(text.as_ptr(), pos_x, pos_y, font_size, color);
  }
}
