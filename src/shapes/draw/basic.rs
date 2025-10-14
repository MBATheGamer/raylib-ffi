use crate::{
  shapes::ffi::{DrawCircle, DrawCircleV, DrawRectangle, DrawTriangle},
  structs::{Color, Vector2},
};

#[inline]
pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
  unsafe { DrawCircle(center_x, center_y, radius, color) };
}

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

#[inline]
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
  unsafe { DrawTriangle(v1, v2, v3, color) };
}
