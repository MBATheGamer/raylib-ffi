use crate::{
  core::ffi::{GetMousePosition, GetMouseWheelMove},
  structs::Vector2,
};

#[inline]
pub fn get_mouse_position() -> Vector2 {
  return unsafe { GetMousePosition() };
}

#[inline]
pub fn get_mouse_wheel_move() -> f32 {
  return unsafe { GetMouseWheelMove() };
}
