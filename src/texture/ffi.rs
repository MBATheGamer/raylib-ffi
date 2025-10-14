use crate::structs::Texture2D;

unsafe extern "C" {
  // Texture loading functions
  pub unsafe fn LoadTexture(filename: *const i8) -> Texture2D;
}
