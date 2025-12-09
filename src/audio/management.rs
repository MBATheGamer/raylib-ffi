use crate::audio::ffi::InitAudioDevice;

#[inline]
pub fn init_audio_device() {
  unsafe { InitAudioDevice() };
}
