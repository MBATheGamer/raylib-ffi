use crate::{
  core::ffi::{BeginDrawing, ClearBackground},
  structs::Color,
};

#[inline]
pub fn clear_background(color: Color) {
  unsafe {
    ClearBackground(color);
  }
}

#[inline]
pub fn begin_drawing() {
  unsafe {
    BeginDrawing();
  }
}
