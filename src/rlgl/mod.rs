use crate::rlgl::ffi::{rlPopMatrix, rlPushMatrix, rlRotatef, rlTranslatef};

mod ffi;

#[inline]
pub fn rl_push_matrix() {
  unsafe { rlPushMatrix() };
}

#[inline]
pub fn rl_pop_matrix() {
  unsafe { rlPopMatrix() };
}

#[inline]
pub fn rl_rotate(angle: f32, x: f32, y: f32, z: f32) {
  unsafe { rlRotatef(angle, x, y, z) };
}

#[inline]
pub fn rl_translate(x: f32, y: f32, z: f32) {
  unsafe { rlTranslatef(x, y, z) };
}
