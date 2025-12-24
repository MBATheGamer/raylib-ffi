use std::f32::EPSILON;

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

  // Divide two quaternions
  #[inline]
  pub fn divide(self, q2: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x / q2.x,
      y: self.y / q2.y,
      z: self.z / q2.z,
      w: self.w / q2.w,
    };
  }

  // Calculate linear interpolation between two quaternions
  #[inline]
  pub fn lerp(self, q2: Quaternion, amount: f32) -> Quaternion {
    return Quaternion {
      x: self.x + amount * (q2.x - self.x),
      y: self.y + amount * (q2.y - self.y),
      z: self.z + amount * (q2.z - self.z),
      w: self.w + amount * (q2.w - self.w),
    };
  }

  // Calculate slerp-optimized interpolation between two quaternions
  #[inline]
  pub fn nlerp(self, q2: Quaternion, amount: f32) -> Quaternion {
    // QuaternionLerp(q1, q2, amount)
    let q = Quaternion {
      x: self.x + amount * (q2.x - self.x),
      y: self.y + amount * (q2.y - self.y),
      z: self.z + amount * (q2.z - self.z),
      w: self.w + amount * (q2.w - self.w),
    };

    // QuaternionNormalize(q);
    let length = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    let length = if length == 0.0 { 1.0 } else { 1.0 / length };

    return Quaternion {
      x: q.x * length,
      y: q.y * length,
      z: q.z * length,
      w: q.w * length,
    };
  }

  // Calculates spherical linear interpolation between two quaternions
  #[inline]
  pub fn slerp(self, mut q2: Quaternion, amount: f32) -> Quaternion {
    let mut cos_half_theta = self.x * q2.x + self.y * q2.y + self.z * q2.z + self.w * q2.w;

    if cos_half_theta < 0.0 {
      q2.x = -q2.x;
      q2.y = -q2.y;
      q2.z = -q2.z;
      q2.w = -q2.w;
      cos_half_theta = -cos_half_theta;
    }

    if cos_half_theta.abs() >= 1.0 {
      return self;
    } else if cos_half_theta > 0.95 {
      return self.nlerp(q2, amount);
    } else {
      let half_theta = cos_half_theta.acos();
      let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

      if sin_half_theta.abs() < EPSILON {
        return Quaternion {
          x: self.x * 0.5 + q2.x * 0.5,
          y: self.y * 0.5 + q2.y * 0.5,
          z: self.z * 0.5 + q2.z * 0.5,
          w: self.w * 0.5 + q2.w * 0.5,
        };
      } else {
        let ratio_a = ((1.0 - amount) * half_theta).sin() / sin_half_theta;
        let ratio_b = (amount * half_theta).sin() / sin_half_theta;

        return Quaternion {
          x: self.x * ratio_a + q2.x * ratio_b,
          y: self.y * ratio_a + q2.y * ratio_b,
          z: self.z * ratio_a + q2.z * ratio_b,
          w: self.w * ratio_a + q2.w * ratio_b,
        };
      }
    }
  }
}
