use crate::structs::{Color, Rectangle, Vector2};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub unsafe fn DrawCircle(center_x: i32, center_y: i32, radius: f32, color: Color);
  pub unsafe fn DrawCircleV(center: Vector2, radius: f32, color: Color);
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
  pub unsafe fn DrawRectangleRec(rectangle: Rectangle, color: Color);
  pub unsafe fn DrawRectangleLines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
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
