use crate::{camera::ffi::CameraYaw, structs::Camera3D};

mod ffi;

impl Camera3D {
  #[inline]
  pub fn camera_yaw(&mut self, angle: f32, rotate_around_target: bool) {
    unsafe { CameraYaw(self, angle, rotate_around_target) };
  }
}
