use crate::{
  shape::ffi::{CheckCollisionPointCircle, CheckCollisionPointRec},
  structs::{Rectangle, Vector2},
};

#[inline]
pub fn check_collision_point_rec(point: Vector2, rectangle: Rectangle) -> bool {
  return unsafe { CheckCollisionPointRec(point, rectangle) };
}

#[inline]
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
  return unsafe { CheckCollisionPointCircle(point, center, radius) };
}
