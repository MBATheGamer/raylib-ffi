use std::ops::Sub;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn sub(&self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}

impl Sub for Vector2 {
  type Output = Vector2;

  fn sub(self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}
