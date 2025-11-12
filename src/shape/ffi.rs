use crate::structs::{Color, Rectangle, Vector2};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub unsafe fn DrawLine(
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: Color,
  );
  pub unsafe fn DrawLineV(start_pos: Vector2, end_pos: Vector2, color: Color);
  pub unsafe fn DrawLineEx(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color);
  pub unsafe fn DrawCircle(center_x: i32, center_y: i32, radius: f32, color: Color);
  pub unsafe fn DrawCircleGradient(
    center_x: i32,
    center_y: i32,
    radius: f32,
    inner: Color,
    outer: Color,
  );
  pub unsafe fn DrawCircleV(center: Vector2, radius: f32, color: Color);
  pub unsafe fn DrawCircleLines(center_x: i32, center_y: i32, radius: f32, color: Color);
  pub unsafe fn DrawRing(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
  );
  pub unsafe fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
  pub unsafe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
  pub unsafe fn DrawRectangleRec(rectangle: Rectangle, color: Color);
  pub unsafe fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
  pub unsafe fn DrawRectangleLines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
  pub unsafe fn DrawRectangleLinesEx(rec: Rectangle, line_thick: f32, color: Color);
  pub unsafe fn DrawRectangleRounded(
    rectangle: Rectangle,
    roundness: f32,
    segments: i32,
    color: Color,
  );
  pub unsafe fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);

  // Basic shapes collision detection functions
  pub unsafe fn CheckCollisionPointRec(point: Vector2, rectangle: Rectangle) -> bool;
}
