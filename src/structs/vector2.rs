use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::structs::Matrix;

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
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
  pub fn distance(&self, other: Vector2) -> f32 {
    return ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
      .sqrt();
  }

  #[inline]
  pub fn distance_sqr(&self, other: Vector2) -> f32 {
    return (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y);
  }

  #[inline]
  pub fn angle(v1: Vector2, v2: Vector2) -> f32 {
    let dot = v1.x * v2.x + v1.y * v2.y;
    let det = v1.x * v2.y - v1.y * v2.x;

    return det.atan2(dot);
  }

  #[inline]
  pub fn scale(&self, scaler: f32) -> Self {
    return Vector2 {
      x: self.x * scaler,
      y: self.y * scaler,
    };
  }

  #[inline]
  pub fn multiply(&self, other: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x * other.x,
      y: self.y * other.y,
    };
  }

  #[inline]
  pub fn negate(&self) -> Vector2 {
    return Vector2 {
      x: -self.x,
      y: -self.y,
    };
  }

  #[inline]
  pub fn divide(self, other: Vector2) -> Vector2 {
    return Vector2 {
      x: self.x / other.x,
      y: self.y / other.y,
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
  pub fn transform(&self, matrix: Matrix) -> Vector2 {
    let x = self.x;
    let y = self.y;
    let z = 0.0;

    return Vector2 {
      x: matrix.m0 * x + matrix.m4 * y + matrix.m8 * z + matrix.m12,
      y: matrix.m1 * x + matrix.m5 * y + matrix.m9 * z + matrix.m13,
    };
  }

  #[inline]
  pub fn lerp(&self, other: Vector2, amount: f32) -> Vector2 {
    return Vector2 {
      x: self.x + amount * (other.x - self.x),
      y: self.y + amount * (other.y - self.y),
    };
  }

  #[inline]
  pub fn reflect(&self, normal: Vector2) -> Vector2 {
    let dot_product = self.x * normal.x + self.y * normal.y; // Dot product

    return Vector2 {
      x: self.x - (2.0 * normal.x) * dot_product,
      y: self.y - (2.0 * normal.y) * dot_product,
    };
  }

  #[inline]
  pub fn rotate(&self, angle: f32) -> Vector2 {
    let cosres = angle.cos();
    let sinres = angle.sin();

    return Vector2 {
      x: self.x * cosres - self.y * sinres,
      y: self.x * sinres + self.y * cosres,
    };
  }

  #[inline]
  pub fn move_towards(&self, target: Vector2, max_distance: f32) -> Vector2 {
    let dx = target.x - self.x;
    let dy = target.y - self.y;
    let value = (dx * dx) + (dy * dy);

    if value == 0.0 || (max_distance >= 0.0 && value <= max_distance * max_distance) {
      return target;
    }

    let dist = value.sqrt();

    return Vector2 {
      x: self.x + dx / dist * max_distance,
      y: self.y + dy / dist * max_distance,
    };
  }

  #[inline]
  pub fn invert(&self) -> Vector2 {
    return Vector2 {
      x: 1.0 / self.x,
      y: 1.0 / self.y,
    };
  }

  #[inline]
  pub fn clamp(&self, min: Vector2, max: Vector2) -> Vector2 {
    return Vector2 {
      x: max.x.min(min.x.max(self.x)),
      y: max.y.min(min.y.max(self.y)),
    };
  }

  #[inline]
  pub fn clamp_value(self, min: f32, max: f32) -> Vector2 {
    let mut length = (self.x * self.x) + (self.y * self.y);
    if length > 0.0 {
      length = length.sqrt();

      let scale = if length < min {
        min / length
      } else if length > max {
        max / length
      } else {
        1.0
      };

      return Vector2 {
        x: self.x * scale,
        y: self.y * scale,
      };
    }

    return self;
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

impl Mul for Vector2 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    return Vector2 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
    };
  }
}

impl Neg for Vector2 {
  type Output = Self;

  fn neg(self) -> Self {
    return Vector2 {
      x: -self.x,
      y: -self.y,
    };
  }
}

impl Div for Vector2 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    return Vector2 {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
    };
  }
}
