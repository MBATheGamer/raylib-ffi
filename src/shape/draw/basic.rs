use crate::{
  shape::ffi::{
    DrawCircle, DrawCircleGradient, DrawCircleLines, DrawCircleV, DrawEllipse, DrawEllipseLines,
    DrawLine, DrawLineEx, DrawLineV, DrawPoly, DrawPolyLines, DrawPolyLinesEx, DrawRectangle,
    DrawRectangleGradientV, DrawRectangleLines, DrawRectangleLinesEx, DrawRectanglePro,
    DrawRectangleRec, DrawRectangleRounded, DrawRectangleV, DrawRing, DrawTriangle,
    DrawTriangleLines,
  },
  structs::{Color, Rectangle, Vector2},
};

#[inline]
pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
  unsafe { DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) };
}

#[inline]
pub fn draw_line_v(start_pos: Vector2, end_pos: Vector2, color: Color) {
  unsafe { DrawLineV(start_pos, end_pos, color) };
}

#[inline]
pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
  unsafe { DrawLineEx(start_pos, end_pos, thick, color) };
}

#[inline]
pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
  unsafe { DrawCircle(center_x, center_y, radius, color) };
}

#[inline]
pub fn draw_circle_gradient(center_x: i32, center_y: i32, radius: f32, inner: Color, outer: Color) {
  unsafe { DrawCircleGradient(center_x, center_y, radius, inner, outer) };
}

#[inline]
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
  unsafe {
    DrawCircleV(center, radius, color);
  }
}

#[inline]
pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
  unsafe { DrawCircleLines(center_x, center_y, radius, color) };
}

#[inline]
pub fn draw_ellipse(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) {
  unsafe { DrawEllipse(center_x, center_y, radius_h, radius_v, color) };
}

#[inline]
pub fn draw_ellipse_lines(
  center_x: i32,
  center_y: i32,
  radius_h: f32,
  radius_v: f32,
  color: Color,
) {
  unsafe { DrawEllipseLines(center_x, center_y, radius_h, radius_v, color) };
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
pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
  unsafe { DrawRectangleV(position, size, color) };
}

#[inline]
pub fn draw_rectangle_rec(rectangle: Rectangle, color: Color) {
  unsafe { DrawRectangleRec(rectangle, color) };
}

#[inline]
pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
  unsafe { DrawRectanglePro(rec, origin, rotation, color) };
}

#[inline]
pub fn draw_rectangle_gradient_v(
  pos_x: i32,
  pos_y: i32,
  width: i32,
  height: i32,
  top: Color,
  bottom: Color,
) {
  unsafe { DrawRectangleGradientV(pos_x, pos_y, width, height, top, bottom) };
}

#[inline]
pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
  unsafe { DrawRectangleLines(pos_x, pos_y, width, height, color) };
}

#[inline]
pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color) {
  unsafe { DrawRectangleLinesEx(rec, line_thick, color) };
}

#[inline]
pub fn draw_rectangle_rounded(rectangle: Rectangle, roundness: f32, segments: i32, color: Color) {
  unsafe { DrawRectangleRounded(rectangle, roundness, segments, color) };
}

#[inline]
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
  unsafe { DrawTriangle(v1, v2, v3, color) };
}

#[inline]
pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
  unsafe { DrawTriangleLines(v1, v2, v3, color) };
}

#[inline]
pub fn draw_poly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
  unsafe { DrawPoly(center, sides, radius, rotation, color) };
}

#[inline]
pub fn draw_poly_lines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
  unsafe { DrawPolyLines(center, sides, radius, rotation, color) };
}

#[inline]
pub fn draw_poly_lines_ex(
  center: Vector2,
  sides: i32,
  radius: f32,
  rotation: f32,
  line_thick: f32,
  color: Color,
) {
  unsafe { DrawPolyLinesEx(center, sides, radius, rotation, line_thick, color) };
}
