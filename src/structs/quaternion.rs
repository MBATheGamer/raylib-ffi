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

  // Get identity quaternion
  #[inline]
  pub fn identity() -> Quaternion {
    return Quaternion {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 1.0,
    };
  }

  // Computes the length of a quaternion
  #[inline]
  pub fn length(self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
  }

  // Normalize provided quaternion
  #[inline]
  pub fn normalize(self) -> Quaternion {
    let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
    let length = if length == 0.0 { 1.0 } else { 1.0 / length };

    return Quaternion {
      x: self.x * length,
      y: self.y * length,
      z: self.z * length,
      w: self.w * length,
    };
  }

  // Invert provided quaternion
  #[inline]
  pub fn invert(self) -> Quaternion {
    let length = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;

    if length != 0.0 {
      let length = 1.0 / length;

      return Quaternion {
        x: self.x * -length,
        y: self.y * -length,
        z: self.z * -length,
        w: self.w * length,
      };
    }

    return self;
  }

  // Calculate two quaternion multiplication
  #[inline]
  pub fn multiply(self, q2: Quaternion) -> Quaternion {
    let (qax, qay, qaz, qaw) = (self.x, self.y, self.z, self.w);
    let (qbx, qby, qbz, qbw) = (q2.x, q2.y, q2.z, q2.w);

    return Quaternion {
      x: qax * qbw + qaw * qbx + qay * qbz - qaz * qby,
      y: qay * qbw + qaw * qby + qaz * qbx - qax * qbz,
      z: qaz * qbw + qaw * qbz + qax * qby - qay * qbx,
      w: qaw * qbw - qax * qbx - qay * qby - qaz * qbz,
    };
  }

  // Scale quaternion by float value
  #[inline]
  pub fn scale(self, scaler: f32) -> Quaternion {
    return Quaternion {
      x: self.x * scaler,
      y: self.y * scaler,
      z: self.z * scaler,
      w: self.w * scaler,
    };
  }
}
