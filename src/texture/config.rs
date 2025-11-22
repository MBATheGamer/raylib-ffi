use crate::{enums::TextureFilter, structs::Texture, texture::ffi::SetTextureFilter};

#[inline]
pub fn set_texture_filter(texture: Texture, filter: TextureFilter) {
  unsafe { SetTextureFilter(texture, filter as i32) };
}
