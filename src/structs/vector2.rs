use std::ops::{Add, Mul, Sub};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  #[inline]
  pub fn zero() -> Vector2 {
    return Vector2 { x: 0.0, y: 0.0 };
  }

  #[inline]
  pub fn one() -> Vector2 {
    return Vector2 { x: 1.0, y: 1.0 };
  }

  #[inline]
  pub fn add(&self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }

  #[inline]
  pub fn add_value(&self, add: f32) -> Vector2 {
    return Vector2 {
      x: self.x + add,
      y: self.y + add,
    };
  }

  #[inline]
  pub fn sub(&self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }

  #[inline]
  pub fn sub_value(&self, sub: f32) -> Vector2 {
    return Vector2 {
      x: self.x - sub,
      y: self.y - sub,
    };
  }

  #[inline]
  pub fn length(&self) -> f32 {
    return (self.x * self.x + self.y * self.y).sqrt();
  }

  #[inline]
  pub fn length_sqr(&self) -> f32 {
    return self.x * self.x + self.y * self.y;
  }

  #[inline]
  pub fn dot_product(&self, other: Vector2) -> f32 {
    return self.x * other.x + self.y * other.y;
  }

  #[inline]
  pub fn scale(&self, scaler: f32) -> Self {
    return Vector2 {
      x: self.x * scaler,
      y: self.y * scaler,
    };
  }

  #[inline]
  pub fn normalize(&self) -> Vector2 {
    let length = (self.x * self.x + self.y * self.y).sqrt();

    if length > 0.0 {
      let length = 1.0 / length;
      return Vector2 {
        x: self.x * length,
        y: self.y * length,
      };
    }

    return Vector2 { x: 0.0, y: 0.0 };
  }

  #[inline]
  pub fn clamp(&self, min: Vector2, max: Vector2) -> Vector2 {
    return Vector2 {
      x: max.x.min(min.x.max(self.x)),
      y: max.y.min(min.y.max(self.y)),
    };
  }
}

impl Add for Vector2 {
  type Output = Vector2;

  fn add(self, rhs: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
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
