use crate::{audio::ffi::PlaySound, structs::Sound};

pub fn play_sound(sound: Sound) {
  unsafe { PlaySound(sound) };
}
