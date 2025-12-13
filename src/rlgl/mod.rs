use crate::rlgl::ffi::{rlPopMatrix, rlPushMatrix, rlRotatef, rlTranslatef};

mod ffi;

pub fn rl_push_matrix() {
  unsafe { rlPushMatrix() };
}

pub fn rl_pop_matrix() {
  unsafe { rlPopMatrix() };
}

pub fn rl_rotate(angle: f32, x: f32, y: f32, z: f32) {
  unsafe { rlRotatef(angle, x, y, z) };
}

pub fn rl_translate(x: f32, y: f32, z: f32) {
  unsafe { rlTranslatef(x, y, z) };
}
