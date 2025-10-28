use std::ffi::{CStr, CString};

use crate::{
  core::ffi::{
    ClearWindowState, CloseWindow, GetCurrentMonitor, GetMonitorCount, GetMonitorHeight,
    GetMonitorName, GetMonitorPhysicalHeight, GetMonitorPhysicalWidth, GetMonitorPosition,
    GetMonitorRefreshRate, GetMonitorWidth, GetScreenHeight, GetScreenWidth, IsWindowState,
    MaximizeWindow, MinimizeWindow, RestoreWindow, SetWindowMonitor, SetWindowState,
    ToggleBorderlessWindowed, ToggleFullscreen, WindowShouldClose,
  },
  enums::ConfigFlags,
  structs::Vector2,
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
pub fn set_window_state(flags: ConfigFlags) {
  unsafe { SetWindowState(flags as u32) };
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
pub fn toggle_borderless_windowed() {
  unsafe { ToggleBorderlessWindowed() };
}

#[inline]
pub fn maximize_window() {
  unsafe { MaximizeWindow() };
}

#[inline]
pub fn minimize_window() {
  unsafe { MinimizeWindow() };
}

#[inline]
pub fn restore_window() {
  unsafe { RestoreWindow() };
}

#[inline]
pub fn set_window_monitor(monitor: i32) {
  unsafe { SetWindowMonitor(monitor) };
}

#[inline]
pub fn get_screen_width() -> i32 {
  return unsafe { GetScreenWidth() };
}

#[inline]
pub fn get_screen_height() -> i32 {
  return unsafe { GetScreenHeight() };
}

#[inline]
pub fn get_monitor_count() -> i32 {
  return unsafe { GetMonitorCount() };
}

#[inline]
pub fn get_current_monitor() -> i32 {
  return unsafe { GetCurrentMonitor() };
}

#[inline]
pub fn get_monitor_position(monitor: i32) -> Vector2 {
  return unsafe { GetMonitorPosition(monitor) };
}

#[inline]
pub fn get_monitor_width(monitor: i32) -> i32 {
  return unsafe { GetMonitorWidth(monitor) };
}

#[inline]
pub fn get_monitor_height(monitor: i32) -> i32 {
  return unsafe { GetMonitorHeight(monitor) };
}

#[inline]
pub fn get_monitor_physical_width(monitor: i32) -> i32 {
  return unsafe { GetMonitorPhysicalWidth(monitor) };
}

#[inline]
pub fn get_monitor_physical_height(monitor: i32) -> i32 {
  return unsafe { GetMonitorPhysicalHeight(monitor) };
}

#[inline]
pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
  return unsafe { GetMonitorRefreshRate(monitor) };
}

#[inline]
pub fn get_monitor_name(monitor: i32) -> &'static str {
  let name = unsafe { CStr::from_ptr(GetMonitorName(monitor)) };

  return name.to_str().expect("C string is not valid UTF-8");
}
