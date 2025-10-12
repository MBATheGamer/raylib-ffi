use crate::core::ffi::{IsCursorHidden, ShowCursor};

#[inline]
pub fn show_cursor() {
  unsafe { ShowCursor() };
}

#[inline]
pub fn is_cursor_hidden() -> bool {
  return unsafe { IsCursorHidden() };
}
