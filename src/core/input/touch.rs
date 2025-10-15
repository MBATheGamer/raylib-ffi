use crate::{
  core::ffi::{GetTouchPointCount, GetTouchPosition},
  structs::Vector2,
};

#[inline]
pub fn get_touch_position(index: i32) -> Vector2 {
  return unsafe { GetTouchPosition(index) };
}

#[inline]
pub fn get_touch_point_count() -> i32 {
  return unsafe { GetTouchPointCount() };
}
