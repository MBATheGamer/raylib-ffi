use crate::core::ffi::{HideCursor, IsCursorHidden, ShowCursor};

#[inline]
pub fn show_cursor() {
  unsafe { ShowCursor() };
}

#[inline]
pub fn hide_cursor() {
  unsafe { HideCursor() };
}

#[inline]
pub fn is_cursor_hidden() -> bool {
  return unsafe { IsCursorHidden() };
}
