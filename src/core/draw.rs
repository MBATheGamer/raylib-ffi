use crate::{
  core::ffi::{
    BeginDrawing, BeginMode2D, BeginTextureMode, ClearBackground, EndDrawing, EndMode2D,
    EndTextureMode,
  },
  structs::{Camera2D, Color, RenderTexture2D},
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

#[inline]
pub fn begin_texture_mode(target: RenderTexture2D) {
  unsafe { BeginTextureMode(target) };
}

#[inline]
pub fn end_texture_mode() {
  unsafe { EndTextureMode() };
}
