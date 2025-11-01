use crate::{
  structs::{Color, Rectangle, Texture, Vector2},
  texture::ffi::{DrawTexture, DrawTexturePro, DrawTextureRec},
};

#[inline]
pub fn draw_texture(texture: Texture, pos_x: i32, pos_y: i32, tint: Color) {
  unsafe { DrawTexture(texture, pos_x, pos_y, tint) };
}

#[inline]
pub fn draw_texture_rec(texture: Texture, source: Rectangle, position: Vector2, tint: Color) {
  unsafe { DrawTextureRec(texture, source, position, tint) };
}

#[inline]
pub fn draw_texture_pro(
  texture: Texture,
  source: Rectangle,
  dest: Rectangle,
  origin: Vector2,
  rotation: f32,
  tint: Color,
) {
  unsafe { DrawTexturePro(texture, source, dest, origin, rotation, tint) };
}
