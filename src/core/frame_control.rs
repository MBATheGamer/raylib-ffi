use crate::core::ffi::{PollInputEvents, SwapScreenBuffer};

#[inline]
pub fn swap_screen_buffer() {
  unsafe { SwapScreenBuffer() };
}

#[inline]
pub fn poll_input_events() {
  unsafe { PollInputEvents() };
}
