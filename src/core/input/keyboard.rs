use crate::{
  core::ffi::{IsKeyDown, IsKeyPressed, SetExitKey},
  enums::KeyboardKey,
};

#[inline]
pub fn is_key_pressed(key: KeyboardKey) -> bool {
  return unsafe { IsKeyPressed(key as i32) };
}

#[inline]
pub fn is_key_down(key: KeyboardKey) -> bool {
  return unsafe { IsKeyDown(key as i32) };
}

#[inline]
pub fn set_exit_key(key: KeyboardKey) {
  unsafe { SetExitKey(key as i32) };
}
