use crate::{
  core::ffi::GetScreenToWorldRay,
  structs::{Camera3D, Ray, Vector2},
};

#[inline]
pub fn get_screen_to_world_ray(position: Vector2, camera: Camera3D) -> Ray {
  return unsafe { GetScreenToWorldRay(position, camera) };
}
