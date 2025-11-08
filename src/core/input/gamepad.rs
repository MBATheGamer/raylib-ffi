use std::ffi::CStr;

use crate::{
  core::ffi::{
    GetGamepadAxisCount, GetGamepadAxisMovement, GetGamepadButtonPressed, GetGamepadName,
    IsGamepadAvailable, IsGamepadButtonDown, IsGamepadButtonPressed,
  },
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
pub fn is_gamepad_button_pressed(gamepad: i32, button: GamepadButton) -> bool {
  return unsafe { IsGamepadButtonPressed(gamepad, button as i32) };
}

#[inline]
pub fn is_gamepad_button_down(gamepad: i32, button: GamepadButton) -> bool {
  return unsafe { IsGamepadButtonDown(gamepad, button as i32) };
}

#[inline]
pub fn get_gamepad_button_pressed() -> GamepadButton {
  return match unsafe { GetGamepadButtonPressed() } {
    1 => GamepadButton::LeftFaceUp,
    2 => GamepadButton::LeftFaceRight,
    3 => GamepadButton::LeftFaceDown,
    4 => GamepadButton::LeftFaceLeft,
    5 => GamepadButton::RightFaceUp,
    6 => GamepadButton::RightFaceRight,
    7 => GamepadButton::RightFaceDown,
    8 => GamepadButton::RightFaceLeft,
    9 => GamepadButton::LeftTrigger1,
    10 => GamepadButton::LeftTrigger2,
    11 => GamepadButton::RightTrigger1,
    12 => GamepadButton::RightTrigger2,
    13 => GamepadButton::MiddleLeft,
    14 => GamepadButton::Middle,
    15 => GamepadButton::MiddleRight,
    16 => GamepadButton::LeftThumb,
    17 => GamepadButton::RightThumb,
    _ => GamepadButton::Unknown,
  };
}

#[inline]
pub fn get_gamepad_axis_count(gamepad: i32) -> i32 {
  return unsafe { GetGamepadAxisCount(gamepad) };
}

#[inline]
pub fn get_gamepad_axis_movement(gamepad: i32, axis: GamepadAxis) -> f32 {
  return unsafe { GetGamepadAxisMovement(gamepad, axis as i32) };
}
