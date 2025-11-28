use crate::{
  structs::{Color, Image, Rectangle},
  texture::ffi::{ImageDraw, ImageDrawCircleLines, ImageDrawPixel},
};

#[inline]
pub fn image_draw_pixel(dst: &mut Image, pos_x: i32, pos_y: i32, color: Color) {
  unsafe { ImageDrawPixel(dst, pos_x, pos_y, color) };
}

#[inline]
pub fn image_draw_circle_lines(
  dst: *mut Image,
  center_x: i32,
  center_y: i32,
  radius: i32,
  color: Color,
) {
  unsafe { ImageDrawCircleLines(dst, center_x, center_y, radius, color) };
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
