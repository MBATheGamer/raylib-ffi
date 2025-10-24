use crate::{
  model::ffi::GetRayCollisionBox,
  structs::{BoundingBox, Ray, RayCollision},
};

#[inline]
pub fn get_ray_collision_box(ray: Ray, bounding_box: BoundingBox) -> RayCollision {
  return unsafe { GetRayCollisionBox(ray, bounding_box) };
}
