use std::mem::transmute;

use crate::{core::ffi::GetGestureDetected, enums::Gesture};

pub fn get_gesture_detected() -> Gesture {
  return unsafe { transmute(GetGestureDetected() as i16) };
}
