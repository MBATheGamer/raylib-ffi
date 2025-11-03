use crate::core::ffi::{PollInputEvents, SwapScreenBuffer, WaitTime};

#[inline]
pub fn swap_screen_buffer() {
  unsafe { SwapScreenBuffer() };
}

#[inline]
pub fn poll_input_events() {
  unsafe { PollInputEvents() };
}

#[inline]
pub fn wait_time(seconds: f64) {
  unsafe { WaitTime(seconds) };
}
