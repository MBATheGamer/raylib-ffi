use crate::{
  structs::{Color, Image, Rectangle},
  texture::ffi::{ImageDraw, ImageDrawPixel},
};

#[inline]
pub fn image_draw_pixel(dst: &mut Image, pos_x: i32, pos_y: i32, color: Color) {
  unsafe { ImageDrawPixel(dst, pos_x, pos_y, color) };
}

#[inline]
pub fn image_draw(
  dst: &mut Image,
  src: Image,
  src_rec: Rectangle,
  dst_rec: Rectangle,
  tint: Color,
) {
  unsafe { ImageDraw(dst, src, src_rec, dst_rec, tint) };
}
