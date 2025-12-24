#[derive(Clone, Copy)]
pub struct Quaternion {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Quaternion {
  // Add two quaternions
  #[inline]
  pub fn add(self, q2: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x + q2.x,
      y: self.y + q2.y,
      z: self.z + q2.z,
      w: self.w + q2.w,
    };
  }

  // Add quaternion and float value
  #[inline]
  pub fn add_value(self, add: f32) -> Quaternion {
    return Quaternion {
      x: self.x + add,
      y: self.y + add,
      z: self.z + add,
      w: self.w + add,
    };
  }

  // Subtract two quaternions
  #[inline]
  pub fn subtract(self, q2: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x - q2.x,
      y: self.y - q2.y,
      z: self.z - q2.z,
      w: self.w - q2.w,
    };
  }

  // Subtract quaternion and float value
  #[inline]
  pub fn subtract_value(self, sub: f32) -> Quaternion {
    return Quaternion {
      x: self.x - sub,
      y: self.y - sub,
      z: self.z - sub,
      w: self.w - sub,
    };
  }
}
