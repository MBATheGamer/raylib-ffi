use crate::{
  camera::ffi::{CameraPitch, CameraYaw},
  structs::Camera3D,
};

mod ffi;

impl Camera3D {
  #[inline]
  pub fn camera_yaw(&mut self, angle: f32, rotate_around_target: bool) {
    unsafe { CameraYaw(self, angle, rotate_around_target) };
  }

  #[inline]
  pub fn camera_pitch(
    &mut self,
    angle: f32,
    lock_view: bool,
    rotate_around_target: bool,
    rotate_up: bool,
  ) {
    unsafe { CameraPitch(self, angle, lock_view, rotate_around_target, rotate_up) };
  }
}
