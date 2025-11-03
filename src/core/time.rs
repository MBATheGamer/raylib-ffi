use crate::core::ffi::{GetFPS, GetFrameTime, GetTime, SetTargetFPS};

#[inline]
pub fn set_target_fps(fps: i32) {
  unsafe { SetTargetFPS(fps) };
}

#[inline]
pub fn get_frame_time() -> f32 {
  return unsafe { GetFrameTime() };
}

#[inline]
pub fn get_time() -> f64 {
  return unsafe { GetTime() };
}

#[inline]
pub fn get_fps() -> i32 {
  return unsafe { GetFPS() };
}
