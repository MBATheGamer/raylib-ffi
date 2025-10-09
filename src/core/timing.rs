use crate::core::ffi::SetTargetFPS;

#[inline]
pub fn set_target_fps(fps: i32) {
  unsafe {
    SetTargetFPS(fps);
  }
}
