use crate::{
  shape::ffi::CheckCollisionPointRec,
  structs::{Rectangle, Vector2},
};

#[inline]
pub fn check_collision_point_rec(point: Vector2, rectangle: Rectangle) -> bool {
  return unsafe { CheckCollisionPointRec(point, rectangle) };
}
