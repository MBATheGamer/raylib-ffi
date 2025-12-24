use std::{
  f32::EPSILON,
  ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

use crate::structs::{Matrix, Vector3};

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
  pub fn add(self, rhs: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }

  // Add quaternion and float value
  #[inline]
  pub fn add_value(self, rhs: f32) -> Quaternion {
    return Quaternion {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs,
      w: self.w + rhs,
    };
  }

  // Subtract two quaternions
  #[inline]
  pub fn subtract(self, rhs: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }

  // Subtract quaternion and float value
  #[inline]
  pub fn subtract_value(self, rhs: f32) -> Quaternion {
    return Quaternion {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs,
      w: self.w - rhs,
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
  pub fn multiply(self, rhs: Quaternion) -> Quaternion {
    let (qax, qay, qaz, qaw) = (self.x, self.y, self.z, self.w);
    let (qbx, qby, qbz, qbw) = (rhs.x, rhs.y, rhs.z, rhs.w);

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
  pub fn divide(self, rhs: Quaternion) -> Quaternion {
    return Quaternion {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
      w: self.w / rhs.w,
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

  // Calculate quaternion cubic spline interpolation using Cubic Hermite Spline algorithm
  // as described in the GLTF 2.0 specification: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#interpolation-cubic
  #[inline]
  pub fn cubic_hermite_spline(
    self,
    out_tangent1: Quaternion,
    q2: Quaternion,
    in_tangent2: Quaternion,
    t: f32,
  ) -> Quaternion {
    let t2 = t * t;
    let t3 = t2 * t;
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let p0 = self.scale(h00);
    let m0 = out_tangent1.scale(h10);
    let p1 = q2.scale(h01);
    let m1 = in_tangent2.scale(h11);

    return p0.add(m0).add(p1).add(m1).normalize();
  }

  // Calculate quaternion based on the rotation from one vector to another
  #[inline]
  pub fn from_vector3_to_vector3(from: Vector3, to: Vector3) -> Quaternion {
    let cos2_theta = from.x * to.x + from.y * to.y + from.z * to.z; // Vector3DotProduct(from, to)
    let cross = Vector3 {
      x: from.y * to.z - from.z * to.y,
      y: from.z * to.x - from.x * to.z,
      z: from.x * to.y - from.y * to.x,
    }; // Vector3CrossProduct(from, to)

    let q = Quaternion {
      x: cross.x,
      y: cross.y,
      z: cross.z,
      w: 1.0 + cos2_theta,
    };

    // QuaternionNormalize(q);
    // NOTE: Normalize to essentially nlerp the original and identity to 0.5
    let length = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
    let length = if length == 0.0 { 1.0 } else { 1.0 / length };

    return Quaternion {
      x: q.x * length,
      y: q.y * length,
      z: q.z * length,
      w: q.w * length,
    };
  }

  // Get a matrix for a given quaternion
  #[inline]
  pub fn to_matrix(self) -> Matrix {
    let a2 = self.x * self.x;
    let b2 = self.y * self.y;
    let c2 = self.z * self.z;
    let ac = self.x * self.z;
    let ab = self.x * self.y;
    let bc = self.y * self.z;
    let ad = self.w * self.x;
    let bd = self.w * self.y;
    let cd = self.w * self.z;

    return Matrix {
      m0: 1.0 - 2.0 * (b2 + c2),
      m1: 2.0 * (ab + cd),
      m2: 2.0 * (ac - bd),
      m3: 0.0,
      m4: 2.0 * (ab - cd),
      m5: 1.0 - 2.0 * (a2 + c2),
      m6: 2.0 * (bc + ad),
      m7: 0.0,
      m8: 2.0 * (ac + bd),
      m9: 2.0 * (bc - ad),
      m10: 1.0 - 2.0 * (a2 + b2),
      m11: 0.0,
      m12: 0.0,
      m13: 0.0,
      m14: 0.0,
      m15: 1.0,
    };
  }

  // Get rotation quaternion for an angle and axis
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn from_axis_angle(mut axis: Vector3, mut angle: f32) -> Quaternion {
    let axis_length = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();

    if axis_length != 0.0 {
      angle *= 0.5;

      // Vector3Normalize(axis)
      let length = axis_length;
      let ilength = if length == 0.0 { 1.0 } else { 1.0 / length };
      axis.x *= ilength;
      axis.y *= ilength;
      axis.z *= ilength;

      let sinres = angle.sin();
      let cosres = angle.cos();

      let q = Quaternion {
        x: axis.x * sinres,
        y: axis.y * sinres,
        z: axis.z * sinres,
        w: cosres,
      };

      // QuaternionNormalize(q);
      // Quaternion q = result;
      let length = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
      let ilength = if length == 0.0 { 1.0 } else { 1.0 / length };

      return Quaternion {
        x: q.x * ilength,
        y: q.y * ilength,
        z: q.z * ilength,
        w: q.w * ilength,
      };
    }

    return Quaternion {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 0.0,
    };
  }

  // Get the rotation angle and axis for a given quaternion
  #[inline]
  pub fn to_axis_angle(mut self, out_axis: &mut Vector3, out_angle: &mut f32) {
    if self.w.abs() > 1.0 {
      // QuaternionNormalize(q);
      let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
      let length = if length == 0.0 { 1.0 } else { 1.0 / length };

      self.x = self.x * length;
      self.y = self.y * length;
      self.z = self.z * length;
      self.w = self.w * length;
    }

    let mut res_axis = Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    };
    let res_angle = 2.0 * self.w.acos();
    let den = (1.0 - self.w * self.w).sqrt();

    if den > EPSILON {
      res_axis.x = self.x / den;
      res_axis.y = self.y / den;
      res_axis.z = self.z / den;
    } else {
      // This occurs when the angle is zero.
      // Not a problem: just set an arbitrary normalized axis.
      res_axis.x = 1.0;
    }

    *out_axis = res_axis;
    *out_angle = res_angle;
  }

  // Get the quaternion equivalent to Euler angles
  // NOTE: Rotation order is ZYX
  #[inline]
  pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Quaternion {
    let x0 = (pitch * 0.5).cos();
    let x1 = (pitch * 0.5).sin();
    let y0 = (yaw * 0.5).cos();
    let y1 = (yaw * 0.5).sin();
    let z0 = (roll * 0.5).cos();
    let z1 = (roll * 0.5).sin();

    return Quaternion {
      x: x1 * y0 * z0 - x0 * y1 * z1,
      y: x0 * y1 * z0 + x1 * y0 * z1,
      z: x0 * y0 * z1 - x1 * y1 * z0,
      w: x0 * y0 * z0 + x1 * y1 * z1,
    };
  }

  // Get the Euler angles equivalent to quaternion (roll, pitch, yaw)
  // NOTE: Angles are returned in a Vector3 struct in radians
  #[inline]
  pub fn to_euler(self) -> Vector3 {
    // Roll (x-axis rotation)
    let x0 = 2.0 * (self.w * self.x + self.y * self.z);
    let x1 = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);

    // Pitch (y-axis rotation)
    let mut y = 2.0 * (self.w * self.y - self.z * self.x);
    y = if y > 1.0 { 1.0 } else { y };
    y = if y < -1.0 { -1.0 } else { y };

    // Yaw (z-axis rotation)
    let z0 = 2.0 * (self.w * self.z + self.x * self.y);
    let z1 = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);

    return Vector3 {
      x: x0.atan2(x1),
      y: y.asin(),
      z: z0.atan2(z1),
    };
  }

  // Transform a quaternion given a transformation matrix
  #[inline]
  pub fn transform(self, mat: Matrix) -> Quaternion {
    return Quaternion {
      x: mat.m0 * self.x + mat.m4 * self.y + mat.m8 * self.z + mat.m12 * self.w,
      y: mat.m1 * self.x + mat.m5 * self.y + mat.m9 * self.z + mat.m13 * self.w,
      z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14 * self.w,
      w: mat.m3 * self.x + mat.m7 * self.y + mat.m11 * self.z + mat.m15 * self.w,
    };
  }
}

impl Add for Quaternion {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    return Quaternion {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }
}

impl Add<f32> for Quaternion {
  type Output = Self;

  fn add(self, rhs: f32) -> Self {
    return Quaternion {
      x: self.x + rhs,
      y: self.y + rhs,
      z: self.z + rhs,
      w: self.w + rhs,
    };
  }
}

impl AddAssign for Quaternion {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
    self.w += rhs.w;
  }
}

impl AddAssign<f32> for Quaternion {
  fn add_assign(&mut self, rhs: f32) {
    self.x += rhs;
    self.y += rhs;
    self.z += rhs;
    self.w += rhs;
  }
}

impl Sub for Quaternion {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    return Quaternion {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }
}

impl Sub<f32> for Quaternion {
  type Output = Self;

  fn sub(self, rhs: f32) -> Self {
    return Quaternion {
      x: self.x - rhs,
      y: self.y - rhs,
      z: self.z - rhs,
      w: self.w - rhs,
    };
  }
}

impl SubAssign for Quaternion {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
    self.w -= rhs.w;
  }
}

impl SubAssign<f32> for Quaternion {
  fn sub_assign(&mut self, rhs: f32) {
    self.x -= rhs;
    self.y -= rhs;
    self.z -= rhs;
    self.w -= rhs;
  }
}

impl Mul for Quaternion {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    return Quaternion {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
      w: self.w * rhs.w,
    };
  }
}

impl Mul<f32> for Quaternion {
  type Output = Self;

  fn mul(self, scaler: f32) -> Self {
    return Quaternion {
      x: self.x * scaler,
      y: self.y * scaler,
      z: self.z * scaler,
      w: self.w * scaler,
    };
  }
}

impl MulAssign for Quaternion {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
    self.z *= rhs.z;
    self.w *= rhs.w;
  }
}

impl MulAssign<f32> for Quaternion {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
    self.w *= rhs;
  }
}

impl Div for Quaternion {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    return Quaternion {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
      w: self.w / rhs.w,
    };
  }
}
