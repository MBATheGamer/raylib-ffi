use crate::core::ffi::IsGamepadAvailable;

#[inline]
pub fn is_gamepad_available(gamepad: i32) -> bool {
  return unsafe { IsGamepadAvailable(gamepad) };
}
