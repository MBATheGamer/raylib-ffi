use crate::{
  structs::{Color, NPatchInfo, Rectangle, Texture, Vector2},
  texture::ffi::{
    DrawTexture, DrawTextureEx, DrawTextureNPatch, DrawTexturePro, DrawTextureRec, DrawTextureV,
  },
};

#[inline]
pub fn draw_texture(texture: Texture, pos_x: i32, pos_y: i32, tint: Color) {
  unsafe { DrawTexture(texture, pos_x, pos_y, tint) };
}

#[inline]
pub fn draw_texture_v(texture: Texture, position: Vector2, tint: Color) {
  unsafe { DrawTextureV(texture, position, tint) };
}

#[inline]
pub fn draw_texture_ex(
  texture: Texture,
  position: Vector2,
  rotation: f32,
  scale: f32,
  tint: Color,
) {
  unsafe { DrawTextureEx(texture, position, rotation, scale, tint) };
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

pub fn draw_texture_n_patch(
  texture: Texture,
  n_patch_info: NPatchInfo,
  dest: Rectangle,
  origin: Vector2,
  rotation: f32,
  tint: Color,
) {
  unsafe { DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint) };
}
