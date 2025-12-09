use crate::structs::Sound;

unsafe extern "C" {
  // Audio device management functions
  pub unsafe fn InitAudioDevice();
  pub unsafe fn CloseAudioDevice();

  // Wave/Sound loading/unloading functions
  pub unsafe fn LoadSound(file_name: *const i8) -> Sound;
  pub unsafe fn UnloadSound(sound: Sound);

  // Wave/Sound management functions
  pub unsafe fn PlaySound(sound: Sound);
}
