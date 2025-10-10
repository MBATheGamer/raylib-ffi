use crate::core::ffi::GetMouseWheelMove;

#[inline]
pub fn get_mouse_wheel_move() -> f32 {
  return unsafe { GetMouseWheelMove() };
}
