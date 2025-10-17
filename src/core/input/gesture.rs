use std::mem::transmute;

use crate::{
  core::ffi::{GetGestureDetected, GetGestureDragAngle},
  enums::Gesture,
};

#[inline]
pub fn get_gesture_detected() -> Gesture {
  return unsafe { transmute(GetGestureDetected() as i16) };
}

#[inline]
pub fn get_gesture_drag_angle() -> f32 {
  return unsafe { GetGestureDragAngle() };
}
