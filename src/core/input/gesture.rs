use std::mem::transmute;

use crate::{
  core::ffi::{GetGestureDetected, GetGestureDragAngle, GetGesturePinchAngle},
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

#[inline]
pub fn get_gesture_pinch_angle() -> f32 {
  return unsafe { GetGesturePinchAngle() };
}
