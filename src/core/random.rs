use crate::core::ffi::GetRandomValue;

#[inline]
pub fn get_random_value(min: i32, max: i32) -> i32 {
  return unsafe { GetRandomValue(min, max) };
}
