use crate::{
  core::ffi::{BeginDrawing, ClearBackground, EndDrawing},
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

#[inline]
pub fn end_drawing() {
  unsafe {
    EndDrawing();
  }
}
