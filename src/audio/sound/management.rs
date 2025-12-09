use crate::{audio::ffi::PlaySound, structs::Sound};

#[inline]
pub fn play_sound(sound: Sound) {
  unsafe { PlaySound(sound) };
}
