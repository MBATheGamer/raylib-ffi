use std::ffi::CString;

use crate::{
  structs::Font,
  text::ffi::{GetFontDefault, LoadFont, LoadFontEx, UnloadFont},
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
pub fn load_font_ex(
  file_name: &str,
  font_size: i32,
  codepoints: Option<Vec<i32>>,
  codepoint_count: i32,
) -> Font {
  let file_name = CString::new(file_name).expect("Expecting font path!");

  return unsafe {
    LoadFontEx(
      file_name.as_ptr(),
      font_size,
      match codepoints {
        Some(codepoints) => codepoints.as_ptr(),
        None => std::ptr::null(),
      },
      codepoint_count,
    )
  };
}

#[inline]
pub fn unload_font(font: Font) {
  unsafe { UnloadFont(font) };
}
