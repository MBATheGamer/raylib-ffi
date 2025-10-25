use crate::{
  core::ffi::{GetScreenToWorldRay, GetWorldToScreen},
  structs::{Camera3D, Ray, Vector2, Vector3},
};

#[inline]
pub fn get_screen_to_world_ray(position: Vector2, camera: Camera3D) -> Ray {
  return unsafe { GetScreenToWorldRay(position, camera) };
}

#[inline]
pub fn get_world_to_screen(position: Vector3, camera: Camera3D) -> Vector2 {
  return unsafe { GetWorldToScreen(position, camera) };
}
