use crate::structs::{Color, RenderTexture2D, Texture2D};

unsafe extern "C" {
  // Texture loading functions
  pub unsafe fn LoadTexture(filename: *const i8) -> Texture2D;
  pub unsafe fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture2D;
  pub unsafe fn UnloadTexture(texture: Texture2D);

  // Texture drawing functions
  pub unsafe fn DrawTexture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color);

  // Color/pixel related functions
  pub unsafe fn Fade(color: Color, alpha: f32) -> Color;
}
