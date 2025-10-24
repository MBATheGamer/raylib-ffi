use crate::core::ffi::{DisableCursor, EnableCursor, HideCursor, IsCursorHidden, ShowCursor};

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

#[inline]
pub fn enable_cursor() {
  unsafe { EnableCursor() };
}

#[inline]
pub fn disable_cursor() {
  unsafe { DisableCursor() };
}
