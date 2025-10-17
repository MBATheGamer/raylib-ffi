use crate::{
  core::ffi::{GetMousePosition, GetMouseWheelMove, IsMouseButtonPressed, IsMouseButtonReleased},
  enums::MouseButton,
  structs::Vector2,
};

#[inline]
pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
  return unsafe { IsMouseButtonPressed(button as i32) };
}

#[inline]
pub fn is_mouse_button_released(button: MouseButton) -> bool {
  return unsafe { IsMouseButtonReleased(button as i32) };
}

#[inline]
pub fn get_mouse_position() -> Vector2 {
  return unsafe { GetMousePosition() };
}

#[inline]
pub fn get_mouse_wheel_move() -> f32 {
  return unsafe { GetMouseWheelMove() };
}
