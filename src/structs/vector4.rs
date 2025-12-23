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
  pub fn add(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x + v2.x,
      y: self.y + v2.y,
      z: self.z + v2.z,
      w: self.w + v2.w,
    };
  }

  #[inline]
  pub fn add_value(self, add: f32) -> Vector4 {
    return Vector4 {
      x: self.x + add,
      y: self.y + add,
      z: self.z + add,
      w: self.w + add,
    };
  }

  #[inline]
  pub fn subtract(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x - v2.x,
      y: self.y - v2.y,
      z: self.z - v2.z,
      w: self.w - v2.w,
    };
  }

  #[inline]
  pub fn subtract_value(self, add: f32) -> Vector4 {
    return Vector4 {
      x: self.x - add,
      y: self.y - add,
      z: self.z - add,
      w: self.w - add,
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
  pub fn dot_product(self, v2: Vector4) -> f32 {
    return self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w;
  }
}
