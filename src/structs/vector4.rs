use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vector4 {
  #[inline]
  pub fn zero() -> Vector4 {
    return Vector4 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 0.0,
    };
  }

  #[inline]
  pub fn one() -> Vector4 {
    return Vector4 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
      w: 1.0,
    };
  }

  #[inline]
  pub fn add(self, rhs: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }

  #[inline]
  pub fn add_value(self, rhs: f32) -> Vector4 {
    return Vector4 {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs,
      w: self.w + rhs,
    };
  }

  #[inline]
  pub fn subtract(self, rhs: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }

  #[inline]
  pub fn subtract_value(self, rhs: f32) -> Vector4 {
    return Vector4 {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs,
      w: self.w - rhs,
    };
  }

  #[inline]
  pub fn length(self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
  }

  #[inline]
  pub fn length_sqr(self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
  }

  #[inline]
  pub fn dot_product(self, other: Vector4) -> f32 {
    return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
  }

  // Calculate distance between two vectors
  #[inline]
  pub fn distance(self, other: Vector4) -> f32 {
    return ((self.x - other.x) * (self.x - other.x)
      + (self.y - other.y) * (self.y - other.y)
      + (self.z - other.z) * (self.z - other.z)
      + (self.w - other.w) * (self.w - other.w))
      .sqrt();
  }

  // Calculate square distance between two vectors
  #[inline]
  pub fn distance_sqr(self, other: Vector4) -> f32 {
    return (self.x - other.x) * (self.x - other.x)
      + (self.y - other.y) * (self.y - other.y)
      + (self.z - other.z) * (self.z - other.z)
      + (self.w - other.w) * (self.w - other.w);
  }

  #[inline]
  pub fn scale(self, scaler: f32) -> Vector4 {
    return Vector4 {
      x: self.x * scaler,
      y: self.y * scaler,
      z: self.z * scaler,
      w: self.w * scaler,
    };
  }

  // Multiply vector by vector
  #[inline]
  pub fn multiply(self, rhs: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
      w: self.w * rhs.w,
    };
  }

  // Negate vector
  #[inline]
  pub fn negate(self) -> Vector4 {
    return Vector4 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    };
  }

  // Divide vector by vector
  #[inline]
  pub fn divide(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x / v2.x,
      y: self.y / v2.y,
      z: self.z / v2.z,
      w: self.w / v2.w,
    };
  }

  // Normalize provided vector
  #[inline]
  pub fn normalize(self) -> Vector4 {
    let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();

    if length > 0.0 {
      let length = 1.0 / length;
      return Vector4 {
        x: self.x * length,
        y: self.y * length,
        z: self.z * length,
        w: self.w * length,
      };
    }

    return Vector4 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 0.0,
    };
  }

  // Get min value for each pair of components
  #[inline]
  pub fn min(self, other: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
      w: self.w.min(other.w),
    };
  }

  // Get max value for each pair of components
  #[inline]
  pub fn max(self, other: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
      w: self.w.max(other.w),
    };
  }

  // Calculate linear interpolation between two vectors
  #[inline]
  pub fn lerp(self, other: Vector4, amount: f32) -> Vector4 {
    return Vector4 {
      x: self.x + amount * (other.x - self.x),
      y: self.y + amount * (other.y - self.y),
      z: self.z + amount * (other.z - self.z),
      w: self.w + amount * (other.w - self.w),
    };
  }

  // Move Vector towards target
  #[inline]
  pub fn move_towards(self, target: Vector4, max_distance: f32) -> Vector4 {
    let dx = target.x - self.x;
    let dy = target.y - self.y;
    let dz = target.z - self.z;
    let dw = target.w - self.w;
    let value = (dx * dx) + (dy * dy) + (dz * dz) + (dw * dw);

    if value == 0.0 || (max_distance >= 0.0 && value <= max_distance * max_distance) {
      return target;
    }

    let dist = value.sqrt();

    return Vector4 {
      x: self.x + dx / dist * max_distance,
      y: self.y + dy / dist * max_distance,
      z: self.z + dz / dist * max_distance,
      w: self.w + dw / dist * max_distance,
    };
  }

  // Invert the given vector
  #[inline]
  pub fn invert(self) -> Vector4 {
    return Vector4 {
      x: 1.0 / self.x,
      y: 1.0 / self.y,
      z: 1.0 / self.z,
      w: 1.0 / self.w,
    };
  }
}

impl Add for Vector4 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    return Vector4 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }
}

impl Add<f32> for Vector4 {
  type Output = Self;

  fn add(self, rhs: f32) -> Self {
    return Vector4 {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs,
      w: self.w + rhs,
    };
  }
}

impl AddAssign for Vector4 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
    self.w += rhs.w;
  }
}

impl AddAssign<f32> for Vector4 {
  fn add_assign(&mut self, rhs: f32) {
    self.x += rhs;
    self.y += rhs;
    self.z += rhs;
    self.w += rhs;
  }
}

impl Sub for Vector4 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    return Vector4 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }
}

impl Sub<f32> for Vector4 {
  type Output = Self;

  fn sub(self, rhs: f32) -> Self {
    return Vector4 {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs,
      w: self.w - rhs,
    };
  }
}

impl SubAssign for Vector4 {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
    self.w -= rhs.w;
  }
}

impl SubAssign<f32> for Vector4 {
  fn sub_assign(&mut self, rhs: f32) {
    self.x -= rhs;
    self.y -= rhs;
    self.z -= rhs;
    self.w -= rhs;
  }
}

impl Mul for Vector4 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    return Vector4 {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
      w: self.w * rhs.w,
    };
  }
}

impl Mul<f32> for Vector4 {
  type Output = Self;

  fn mul(self, scaler: f32) -> Self {
    return Vector4 {
      x: self.x * scaler,
      y: self.y * scaler,
      z: self.z * scaler,
      w: self.w * scaler,
    };
  }
}

impl MulAssign for Vector4 {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
    self.w *= rhs.w;
  }
}

impl MulAssign<f32> for Vector4 {
  fn mul_assign(&mut self, scaler: f32) {
    self.x *= scaler;
    self.y *= scaler;
    self.z *= scaler;
    self.w *= scaler;
  }
}

impl Neg for Vector4 {
  type Output = Self;

  fn neg(self) -> Self {
    return Vector4 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    };
  }
}
