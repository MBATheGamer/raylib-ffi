use crate::structs::{Color, Rectangle, Vector2};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub fn DrawLine(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color);
  pub fn DrawLineV(start_pos: Vector2, end_pos: Vector2, color: Color);
  pub fn DrawLineEx(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color);
  pub fn DrawLineBezier(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color);
  pub fn DrawLineDashed(
    start_pos: Vector2,
    end_pos: Vector2,
    dash_size: i32,
    space_size: i32,
    color: Color,
  );
  pub fn DrawCircle(center_x: i32, center_y: i32, radius: f32, color: Color);
  pub fn DrawCircleSector(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
  );
  pub fn DrawCircleGradient(center_x: i32, center_y: i32, radius: f32, inner: Color, outer: Color);
  pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);
  pub fn DrawCircleLines(center_x: i32, center_y: i32, radius: f32, color: Color);
  pub fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color);
  pub fn DrawEllipse(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color);
  pub fn DrawEllipseLines(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color);
  pub fn DrawRing(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
  );
  pub fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
  pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
  pub fn DrawRectangleRec(rectangle: Rectangle, color: Color);
  pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
  pub fn DrawRectangleGradientV(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    top: Color,
    bottom: Color,
  );
  pub fn DrawRectangleLines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
  pub fn DrawRectangleLinesEx(rec: Rectangle, line_thick: f32, color: Color);
  pub fn DrawRectangleRounded(rectangle: Rectangle, roundness: f32, segments: i32, color: Color);
  pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
  pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
  pub fn DrawPoly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color);
  pub fn DrawPolyLines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color);
  pub fn DrawPolyLinesEx(
    center: Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    line_thick: f32,
    color: Color,
  );

  // Basic shapes collision detection functions
  pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
  pub fn CheckCollisionPointRec(point: Vector2, rectangle: Rectangle) -> bool;
  pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;
  pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;
}
