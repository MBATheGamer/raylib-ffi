use std::ffi::CString;

use crate::{
  structs::{Color, Font, Image, Rectangle, Vector2},
  texture::ffi::{
    ImageDraw, ImageDrawCircleLines, ImageDrawPixel, ImageDrawRectangle, ImageDrawTextEx,
  },
};

#[inline]
pub fn image_draw_pixel(dst: &mut Image, pos_x: i32, pos_y: i32, color: Color) {
  unsafe { ImageDrawPixel(dst, pos_x, pos_y, color) };
}

#[inline]
pub fn image_draw_circle_lines(
  dst: &mut Image,
  center_x: i32,
  center_y: i32,
  radius: i32,
  color: Color,
) {
  unsafe { ImageDrawCircleLines(dst, center_x, center_y, radius, color) };
}

#[inline]
pub fn image_draw_rectangle(
  dst: &mut Image,
  pos_x: i32,
  pos_y: i32,
  width: i32,
  height: i32,
  color: Color,
) {
  unsafe { ImageDrawRectangle(dst, pos_x, pos_y, width, height, color) };
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

#[inline]
pub fn image_draw_text_ex(
  dst: &mut Image,
  font: Font,
  text: &str,
  position: Vector2,
  font_size: f32,
  spacing: f32,
  tint: Color,
) {
  let text = CString::new(text).expect("Expecting text message!");

  unsafe { ImageDrawTextEx(dst, font, text.as_ptr(), position, font_size, spacing, tint) }
}
