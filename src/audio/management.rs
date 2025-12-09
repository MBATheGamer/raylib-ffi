use crate::audio::ffi::{CloseAudioDevice, InitAudioDevice};

#[inline]
pub fn init_audio_device() {
  unsafe { InitAudioDevice() };
}

#[inline]
pub fn close_audio_device() {
  unsafe { CloseAudioDevice() };
}
