use crate::{
  shapes::ffi::{DrawCircleV, DrawRectangle},
  structs::{Color, Vector2},
};

#[inline]
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
  unsafe {
    DrawCircleV(center, radius, color);
  }
}

#[inline]
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe {
    DrawRectangle(pos_x, pos_y, width, height, color);
  }
}
