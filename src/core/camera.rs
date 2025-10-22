use crate::{core::ffi::UpdateCamera, enums::CameraMode, structs::Camera3D};

#[inline]
pub fn update_camera(camera: &mut Camera3D, mode: CameraMode) {
  unsafe { UpdateCamera(camera, mode as i32) };
}
