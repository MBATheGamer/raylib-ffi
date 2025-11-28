use crate::{structs::Font, text::ffi::GetFontDefault};

#[inline]
pub fn get_font_default() -> Font {
  return unsafe { GetFontDefault() };
}
