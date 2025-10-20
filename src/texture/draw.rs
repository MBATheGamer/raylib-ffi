use crate::{
  structs::{Color, Rectangle, Texture2D, Vector2},
  texture::ffi::{DrawTexture, DrawTextureRec},
};

#[inline]
pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
  unsafe { DrawTexture(texture, pos_x, pos_y, tint) };
}

#[inline]
pub fn draw_texture_rec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color) {
  unsafe { DrawTextureRec(texture, source, position, tint) };
}
