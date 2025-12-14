use crate::structs::Sound;

unsafe extern "C" {
  // Audio device management functions
  pub fn InitAudioDevice();
  pub fn CloseAudioDevice();

  // Wave/Sound loading/unloading functions
  pub fn LoadSound(file_name: *const i8) -> Sound;
  pub fn UnloadSound(sound: Sound);

  // Wave/Sound management functions
  pub fn PlaySound(sound: Sound);
}
