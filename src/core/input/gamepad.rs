use std::ffi::CStr;

use crate::{
  core::ffi::{GetGamepadAxisMovement, GetGamepadName, IsGamepadAvailable, IsGamepadButtonDown},
  enums::{GamepadAxis, GamepadButton},
};

#[inline]
pub fn is_gamepad_available(gamepad: i32) -> bool {
  return unsafe { IsGamepadAvailable(gamepad) };
}

#[inline]
pub fn get_gamepad_name(gamepad: i32) -> &'static str {
  let c_str = unsafe { CStr::from_ptr(GetGamepadName(gamepad)) };

  return c_str.to_str().unwrap();
}

#[inline]
pub fn is_gamepad_button_down(gamepad: i32, button: GamepadButton) -> bool {
  return unsafe { IsGamepadButtonDown(gamepad, button as i32) };
}

#[inline]
pub fn get_gamepad_axis_movement(gamepad: i32, axis: GamepadAxis) -> f32 {
  return unsafe { GetGamepadAxisMovement(gamepad, axis as i32) };
}
