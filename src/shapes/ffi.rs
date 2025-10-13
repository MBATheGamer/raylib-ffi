use crate::structs::{Color, Vector2};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub unsafe fn DrawCircleV(center: Vector2, radius: f32, color: Color);
  pub unsafe fn DrawRectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color);
}
