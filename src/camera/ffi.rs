use crate::structs::Camera3D;

unsafe extern "C" {
  pub fn CameraYaw(camera: *mut Camera3D, angle: f32, rotate_around_target: bool);
}
