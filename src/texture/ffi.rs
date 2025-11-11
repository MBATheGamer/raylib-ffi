use crate::structs::{Color, Image, Rectangle, RenderTexture, Texture, Vector2};

unsafe extern "C" {
  // Image loading functions
  pub unsafe fn IsImageValid(image: Image) -> bool;
  pub unsafe fn UnloadImage(image: Image);

  // Texture loading functions
  pub unsafe fn LoadTexture(filename: *const i8) -> Texture;
  pub unsafe fn LoadRenderTexture(width: i32, height: i32) -> RenderTexture;
  pub unsafe fn UnloadTexture(texture: Texture);
  pub unsafe fn UnloadRenderTexture(target: RenderTexture);

  // Texture drawing functions
  pub unsafe fn DrawTexture(texture: Texture, pos_x: i32, pos_y: i32, tint: Color);
  pub unsafe fn DrawTextureRec(texture: Texture, source: Rectangle, position: Vector2, tint: Color);
  pub unsafe fn DrawTexturePro(
    texture: Texture,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
  );

  // Color/pixel related functions
  pub unsafe fn Fade(color: Color, alpha: f32) -> Color;
  pub unsafe fn ColorAlpha(color: Color, alpha: f32) -> Color;
}
