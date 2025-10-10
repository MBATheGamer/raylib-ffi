use crate::core::ffi::{GetFrameTime, SetTargetFPS};

#[inline]
pub fn set_target_fps(fps: i32) {
  unsafe {
    SetTargetFPS(fps);
  }
}

#[inline]
pub fn get_frame_time() -> f32 {
  return unsafe { GetFrameTime() };
}
