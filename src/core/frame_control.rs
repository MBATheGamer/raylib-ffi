use crate::core::ffi::PollInputEvents;

#[inline]
pub fn poll_input_events() {
  unsafe { PollInputEvents() };
}
