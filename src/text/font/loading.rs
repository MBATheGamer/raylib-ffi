use std::ffi::CString;

use crate::{
  structs::Font,
  text::ffi::{GetFontDefault, LoadFont, UnloadFont},
};

#[inline]
pub fn get_font_default() -> Font {
  return unsafe { GetFontDefault() };
}

#[inline]
pub fn load_font(file_name: &str) -> Font {
  let file_name = CString::new(file_name).expect("Expecting font path!");

  return unsafe { LoadFont(file_name.as_ptr()) };
}

#[inline]
pub fn unload_font(font: Font) {
  unsafe { UnloadFont(font) };
}
