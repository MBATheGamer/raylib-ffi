use std::ops::{Mul, Sub};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn length(&self) -> f32 {
    return (self.x * self.x + self.y * self.y).sqrt();
  }

  pub fn sub(&self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }

  pub fn scale(&self, scaler: f32) -> Self {
    return Vector2 {
      x: self.x * scaler,
      y: self.y * scaler,
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

impl Mul<Vector2> for f32 {
  type Output = Vector2;

  fn mul(self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self * rhs.x,
      y: self * rhs.y,
    };
  }
}

impl Mul<f32> for Vector2 {
  type Output = Vector2;

  fn mul(self, rhs: f32) -> Vector2 {
    return Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    };
  }
}
