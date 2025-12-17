use crate::core::ffi::{GetRandomValue, LoadRandomSequence, UnloadRandomSequence};

#[inline]
pub fn get_random_value(min: i32, max: i32) -> i32 {
  return unsafe { GetRandomValue(min, max) };
}

#[inline]
pub fn load_random_sequence(count: usize, min: i32, max: i32) -> Vec<i32> {
  let ptr = unsafe { LoadRandomSequence(count as u32, min, max) };

  return unsafe { Vec::from_raw_parts(ptr, count, count + 1) };
}

#[inline]
pub fn unload_random_sequence(sequence: &mut Vec<i32>) {
  unsafe { UnloadRandomSequence(sequence.as_mut_ptr()) };
}
