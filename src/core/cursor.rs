use crate::core::ffi::IsCursorHidden;

#[inline]
pub fn is_cursor_hidden() -> bool {
  return unsafe { IsCursorHidden() };
}
