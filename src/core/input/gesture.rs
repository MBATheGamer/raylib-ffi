use std::mem::transmute;

use crate::{
  core::ffi::{GetGestureDetected, GetGestureDragAngle, GetGesturePinchAngle, IsGestureDetected},
  enums::Gesture,
};

#[inline]
pub fn get_gesture_detected() -> Gesture {
  return unsafe { transmute(GetGestureDetected() as i16) };
}

#[inline]
pub fn is_gesture_detected(gesture: Gesture) -> bool {
  return unsafe { IsGestureDetected(gesture as u32) };
}

#[inline]
pub fn get_gesture_drag_angle() -> f32 {
  return unsafe { GetGestureDragAngle() };
}

#[inline]
pub fn get_gesture_pinch_angle() -> f32 {
  return unsafe { GetGesturePinchAngle() };
}
