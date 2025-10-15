use crate::core::ffi::GetTouchPointCount;

#[inline]
pub fn get_touch_point_count() -> i32 {
  return unsafe { GetTouchPointCount() };
}
