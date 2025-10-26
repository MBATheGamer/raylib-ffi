use std::ffi::CString;

use crate::{
  core::ffi::{
    ClearWindowState, CloseWindow, GetScreenHeight, GetScreenWidth, IsWindowState,
    ToggleFullscreen, WindowShouldClose,
  },
  enums::ConfigFlags,
};

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

#[inline]
pub fn is_window_state(flag: ConfigFlags) -> bool {
  return unsafe { IsWindowState(flag as u32) };
}

#[inline]
pub fn clear_window_state(flags: ConfigFlags) {
  unsafe { ClearWindowState(flags as u32) };
}

#[inline]
pub fn toggle_fullscreen() {
  unsafe { ToggleFullscreen() };
}

#[inline]
pub fn get_screen_width() -> i32 {
  return unsafe { GetScreenWidth() };
}

#[inline]
pub fn get_screen_height() -> i32 {
  return unsafe { GetScreenHeight() };
}
