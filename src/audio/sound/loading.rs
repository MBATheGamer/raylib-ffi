use std::ffi::CString;

use crate::{audio::ffi::LoadSound, structs::Sound};

pub fn load_sound(file_name: &str) -> Sound {
  let file_name = CString::new(file_name).expect("Expecting sound file path");

  return unsafe { LoadSound(file_name.as_ptr()) };
}
