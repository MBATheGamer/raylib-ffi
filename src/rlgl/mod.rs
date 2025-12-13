use crate::rlgl::ffi::rlPushMatrix;

mod ffi;

pub fn rl_push_matrix() {
  unsafe { rlPushMatrix() };
}
