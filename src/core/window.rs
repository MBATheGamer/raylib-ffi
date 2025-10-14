use std::ffi::CString;

use crate::core::ffi::{CloseWindow, WindowShouldClose};

use super::ffi::InitWindow;

#[inline]
pub fn init_window(width: i32, height: i32, title: &str) {
  let title = CString::new(title).expect("You must add title to your application");

  unsafe { InitWindow(width, height, title.as_ptr()) };
}

#[inline]
pub fn close_window() {
  unsafe { CloseWindow() };
}

#[inline]
pub fn window_should_close() -> bool {
  return unsafe { WindowShouldClose() };
}
