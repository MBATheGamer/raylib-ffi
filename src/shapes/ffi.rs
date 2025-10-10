use crate::structs::{Color, Vector2};

unsafe extern "C" {
  // Basic shapes drawing functions
  pub unsafe fn DrawCircleV(center: Vector2, radius: f32, color: Color);
}
