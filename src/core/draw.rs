use crate::core::ffi::BeginDrawing;

pub fn begin_drawing() {
  unsafe {
    BeginDrawing();
  }
}
