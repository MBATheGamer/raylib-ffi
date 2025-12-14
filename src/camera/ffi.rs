use crate::structs::{Camera3D, Vector3};

unsafe extern "C" {
  pub fn CameraYaw(camera: *mut Camera3D, angle: f32, rotate_around_target: bool);
  pub fn CameraPitch(
    camera: *mut Camera3D,
    angle: f32,
    lock_view: bool,
    rotate_around_target: bool,
    rotate_up: bool,
  );
  pub fn UpdateCameraPro(camera: &mut Camera3D, movement: Vector3, rotation: Vector3, zoom: f32);
}
