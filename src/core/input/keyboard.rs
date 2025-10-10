use crate::{core::ffi::IsKeyPressed, enums::KeyboardKey};

#[inline]
pub fn is_key_pressed(key: KeyboardKey) -> bool {
  return unsafe { IsKeyPressed(key as i32) };
}
