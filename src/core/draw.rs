use crate::{
  core::ffi::{BeginDrawing, BeginMode2D, ClearBackground, EndDrawing, EndMode2D},
  structs::{Camera2D, Color},
};

#[inline]
pub fn clear_background(color: Color) {
  unsafe { ClearBackground(color) };
}

#[inline]
pub fn begin_drawing() {
  unsafe { BeginDrawing() };
}

#[inline]
pub fn end_drawing() {
  unsafe { EndDrawing() };
}

#[inline]
pub fn begin_mode_2d(camera: Camera2D) {
  unsafe { BeginMode2D(camera) };
}

#[inline]
pub fn end_mode_2d() {
  unsafe { EndMode2D() };
}
