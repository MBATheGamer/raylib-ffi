use crate::{
  shapes::ffi::{
    DrawCircle, DrawCircleV, DrawRectangle, DrawRectangleLines, DrawRectangleRec,
    DrawRectangleRounded, DrawRing, DrawTriangle,
  },
  structs::{Color, Rectangle, Vector2},
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
pub fn draw_ring(
  center: Vector2,
  inner_radius: f32,
  outer_radius: f32,
  start_angle: f32,
  end_angle: f32,
  segments: i32,
  color: Color,
) {
  unsafe {
    DrawRing(
      center,
      inner_radius,
      outer_radius,
      start_angle,
      end_angle,
      segments,
      color,
    );
  };
}

#[inline]
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe {
    DrawRectangle(pos_x, pos_y, width, height, color);
  }
}

#[inline]
pub fn draw_rectangle_rec(rectangle: Rectangle, color: Color) {
  unsafe { DrawRectangleRec(rectangle, color) };
}

#[inline]
pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe { DrawRectangleLines(pos_x, pos_y, width, height, color) };
}

#[inline]
pub fn draw_rectangle_rounded(rectangle: Rectangle, roundness: f32, segments: i32, color: Color) {
  unsafe { DrawRectangleRounded(rectangle, roundness, segments, color) };
}

#[inline]
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
  unsafe { DrawTriangle(v1, v2, v3, color) };
}
