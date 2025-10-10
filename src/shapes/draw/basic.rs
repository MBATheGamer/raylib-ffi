use crate::{
  shapes::ffi::DrawCircleV,
  structs::{Color, Vector2},
};

#[inline]
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
  unsafe {
    DrawCircleV(center, radius, color);
  }
}
