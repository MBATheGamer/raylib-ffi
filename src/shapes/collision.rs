use crate::{
  shapes::ffi::CheckCollisionPointRec,
  structs::{Rectangle, Vector2},
};

pub fn check_collision_point_rec(point: Vector2, rectangle: Rectangle) -> bool {
  return unsafe { CheckCollisionPointRec(point, rectangle) };
}
