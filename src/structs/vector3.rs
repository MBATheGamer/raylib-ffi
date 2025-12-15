use std::ops::{Add, Mul, Neg, Sub};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  #[inline]
  pub fn zero() -> Vector3 {
    return Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    };
  }

  #[inline]
  pub fn one() -> Vector3 {
    return Vector3 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
    };
  }

  #[inline]
  pub fn add(&self, rhs: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }

  #[inline]
  pub fn add_value(&self, add: f32) -> Vector3 {
    return Vector3 {
      x: self.x + add,
      y: self.y + add,
      z: self.z + add,
    };
  }

  #[inline]
  pub fn sub(&self, rhs: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }

  #[inline]
  pub fn sub_value(&self, add: f32) -> Vector3 {
    return Vector3 {
      x: self.x - add,
      y: self.y - add,
      z: self.z - add,
    };
  }

  #[inline]
  pub fn scale(&self, scalar: f32) -> Vector3 {
    return Vector3 {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    };
  }

  #[inline]
  pub fn multiply(&self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    };
  }

  #[inline]
  pub fn cross_product(&self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };
  }

  #[inline]
  pub fn perpendicular(&self) -> Vector3 {
    let mut min = self.x.abs();
    let mut cardinal_axis = Vector3 {
      x: 1.0,
      y: 0.0,
      z: 0.0,
    };

    if self.y.abs() < min {
      min = self.y.abs();
      let tmp = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
      };
      cardinal_axis = tmp;
    }

    if self.z.abs() < min {
      let tmp = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
      };
      cardinal_axis = tmp;
    }

    return Vector3 {
      x: self.y * cardinal_axis.z - self.z * cardinal_axis.y,
      y: self.z * cardinal_axis.x - self.x * cardinal_axis.z,
      z: self.x * cardinal_axis.y - self.y * cardinal_axis.x,
    };
  }

  #[inline]
  pub fn length(&self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
  }

  #[inline]
  pub fn length_sqr(&self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z;
  }

  #[inline]
  pub fn angle(&self, other: Vector3) -> f32 {
    let cross = Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };
    let len = (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt();
    let dot = self.x * other.x + self.y * other.y + self.z * other.z;

    return len.atan2(dot);
  }

  #[inline]
  pub fn dot_product(&self, other: Vector3) -> f32 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }

  #[inline]
  pub fn lerp(&self, other: Vector3, amount: f32) -> Vector3 {
    return Vector3 {
      x: self.x + amount * (other.x - self.x),
      y: self.y + amount * (other.y - self.y),
      z: self.z + amount * (other.z - self.z),
    };
  }

  #[inline]
  pub fn normalize(&self) -> Vector3 {
    let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

    if length != 0.0 {
      let length = 1.0 / length;

      return Vector3 {
        x: self.x * length,
        y: self.y * length,
        z: self.z * length,
      };
    }

    return *self;
  }

  #[inline]
  pub fn rotate_by_axis_angle(&self, mut axis: Vector3, mut angle: f32) -> Vector3 {
    // Using Euler-Rodrigues Formula
    // Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
    let mut result = *self;

    // Vector3Normalize(axis);
    let mut length = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
    if length == 0.0 {
      length = 1.0;
    }

    let length = 1.0 / length;
    axis.x *= length;
    axis.y *= length;
    axis.z *= length;

    angle /= 2.0;
    let mut a = angle.sin();
    let b = axis.x * a;
    let c = axis.y * a;
    let d = axis.z * a;
    a = angle.cos();

    let w = Vector3 { x: b, y: c, z: d };

    // Vector3CrossProduct(w, v)
    let mut w_v = Vector3 {
      x: w.y * self.z - w.z * self.y,
      y: w.z * self.x - w.x * self.z,
      z: w.x * self.y - w.y * self.x,
    };

    // Vector3CrossProduct(w, w_v)
    let mut w_wv = Vector3 {
      x: w.y * w_v.z - w.z * w_v.y,
      y: w.z * w_v.x - w.x * w_v.z,
      z: w.x * w_v.y - w.y * w_v.x,
    };

    // Vector3Scale(w_v, 2*a)
    a *= 2.0;
    w_v.x *= a;
    w_v.y *= a;
    w_v.z *= a;

    // Vector3Scale(w_wv, 2)
    w_wv.x *= 2.0;
    w_wv.y *= 2.0;
    w_wv.z *= 2.0;

    result.x += w_v.x;
    result.y += w_v.y;
    result.z += w_v.z;

    result.x += w_wv.x;
    result.y += w_wv.y;
    result.z += w_wv.z;

    return result;
  }
}

impl Add for Vector3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    return Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }
}

impl Sub for Vector3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    return Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }
}

impl Mul<Vector3> for f32 {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Vector3 {
    return Vector3 {
      x: self * rhs.x,
      y: self * rhs.y,
      z: self * rhs.z,
    };
  }
}

impl Mul<f32> for Vector3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    return Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    };
  }
}

impl Mul for Vector3 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    return Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
    };
  }
}

impl Neg for Vector3 {
  type Output = Self;

  fn neg(self) -> Self {
    return Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    };
  }
}
