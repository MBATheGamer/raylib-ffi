use crate::rlgl::ffi::{rlPushMatrix, rlTranslatef};

mod ffi;

pub fn rl_push_matrix() {
  unsafe { rlPushMatrix() };
}

pub fn rl_translate(x: f32, y: f32, z: f32) {
  unsafe { rlTranslatef(x, y, z) };
}
