use std::ffi::CString;

use crate::text::ffi::MeasureText;

#[inline]
pub fn measure_text(text: &str, font_size: i32) -> i32 {
  let text = CString::new(text).expect("You must set a text in this function");

  return unsafe { MeasureText(text.as_ptr(), font_size) };
}
