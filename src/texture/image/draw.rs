use crate::{
  structs::{Color, Image, Rectangle},
  texture::ffi::ImageDraw,
};

pub fn image_draw(
  dst: &mut Image,
  src: Image,
  src_rec: Rectangle,
  dst_rec: Rectangle,
  tint: Color,
) {
  unsafe { ImageDraw(dst, src, src_rec, dst_rec, tint) };
}
