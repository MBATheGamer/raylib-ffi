use crate::{
  structs::{Color, Texture2D},
  texture::ffi::DrawTexture,
};

#[inline]
pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
  unsafe {
    DrawTexture(texture, pos_x, pos_y, tint);
  }
}
