use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::structs::{Matrix, Quaternion};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
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
  pub fn add(self, rhs: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }

  #[inline]
  pub fn add_value(self, add: f32) -> Vector3 {
    return Vector3 {
      x: self.x + add,
      y: self.y + add,
      z: self.z + add,
    };
  }

  #[inline]
  pub fn sub(self, rhs: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }

  #[inline]
  pub fn sub_value(self, add: f32) -> Vector3 {
    return Vector3 {
      x: self.x - add,
      y: self.y - add,
      z: self.z - add,
    };
  }

  #[inline]
  pub fn scale(self, scalar: f32) -> Vector3 {
    return Vector3 {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    };
  }

  #[inline]
  pub fn multiply(self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    };
  }

  #[inline]
  pub fn cross_product(self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };
  }

  #[inline]
  pub fn perpendicular(self) -> Vector3 {
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
  pub fn length(self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
  }

  #[inline]
  pub fn length_sqr(self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z;
  }

  #[inline]
  pub fn dot_product(self, other: Vector3) -> f32 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }

  #[inline]
  pub fn distance(self, other: Vector3) -> f32 {
    let dx = other.x - self.x;
    let dy = other.y - self.y;
    let dz = other.z - self.z;

    return (dx * dx + dy * dy + dz * dz).sqrt();
  }

  #[inline]
  pub fn distance_sqr(self, other: Vector3) -> f32 {
    let dx = other.x - self.x;
    let dy = other.y - self.y;
    let dz = other.z - self.z;

    return dx * dx + dy * dy + dz * dz;
  }

  #[inline]
  pub fn angle(self, other: Vector3) -> f32 {
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
  pub fn negate(self) -> Vector3 {
    return Vector3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    };
  }

  #[inline]
  pub fn divide(self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z,
    };
  }

  #[inline]
  pub fn normalize(self) -> Vector3 {
    let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

    if length != 0.0 {
      let length = 1.0 / length;

      return Vector3 {
        x: self.x * length,
        y: self.y * length,
        z: self.z * length,
      };
    }

    return self;
  }

  #[inline]
  pub fn project(self, other: Vector3) -> Vector3 {
    let mag = (self.x * other.x + self.y * other.y + self.z * other.z)
      / (other.x * other.x + other.y * other.y + other.z * other.z);

    return Vector3 {
      x: other.x * mag,
      y: other.y * mag,
      z: other.z * mag,
    };
  }

  #[inline]
  pub fn reject(self, other: Vector3) -> Vector3 {
    let mag = (self.x * other.x + self.y * other.y + self.z * other.z)
      / (other.x * other.x + other.y * other.y + other.z * other.z);

    return Vector3 {
      x: self.x - (other.x * mag),
      y: self.y - (other.y * mag),
      z: self.z - (other.z * mag),
    };
  }

  #[inline]
  pub fn ortho_normalize(&mut self, other: &mut Vector3) {
    // Vector3Normalize(*v1);
    let mut length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    if length == 0.0 {
      length = 1.0;
    }
    length = 1.0 / length;
    self.x *= length;
    self.y *= length;
    self.z *= length;

    // Vector3CrossProduct(*v1, *v2)
    let mut vn1 = Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };

    // Vector3Normalize(vn1);
    length = (vn1.x * vn1.x + vn1.y * vn1.y + vn1.z * vn1.z).sqrt();
    if length == 0.0 {
      length = 1.0;
    }
    length = 1.0 / length;
    vn1.x *= length;
    vn1.y *= length;
    vn1.z *= length;

    // Vector3CrossProduct(vn1, *v1)
    *other = Vector3 {
      x: vn1.y * self.z - vn1.z * self.y,
      y: vn1.z * self.x - vn1.x * self.z,
      z: vn1.x * self.y - vn1.y * self.x,
    };
  }

  #[inline]
  pub fn transform(self, matrix: Matrix) -> Vector3 {
    return Vector3 {
      x: matrix.m0 * self.x + matrix.m4 * self.y + matrix.m8 * self.z + matrix.m12,
      y: matrix.m1 * self.x + matrix.m5 * self.y + matrix.m9 * self.z + matrix.m13,
      z: matrix.m2 * self.x + matrix.m6 * self.y + matrix.m10 * self.z + matrix.m14,
    };
  }

  #[inline]
  pub fn rotate_by_quaternion(self, quaternion: Quaternion) -> Vector3 {
    return Vector3 {
      x: self.x
        * (quaternion.x * quaternion.x + quaternion.w * quaternion.w
          - quaternion.y * quaternion.y
          - quaternion.z * quaternion.z)
        + self.y * (2.0 * quaternion.x * quaternion.y - 2.0 * quaternion.w * quaternion.z)
        + self.z * (2.0 * quaternion.x * quaternion.z + 2.0 * quaternion.w * quaternion.y),
      y: self.x * (2.0 * quaternion.w * quaternion.z + 2.0 * quaternion.x * quaternion.y)
        + self.y
          * (quaternion.w * quaternion.w - quaternion.x * quaternion.x
            + quaternion.y * quaternion.y
            - quaternion.z * quaternion.z)
        + self.z * (-2.0 * quaternion.w * quaternion.x + 2.0 * quaternion.y * quaternion.z),
      z: self.x * (-2.0 * quaternion.w * quaternion.y + 2.0 * quaternion.x * quaternion.z)
        + self.y * (2.0 * quaternion.w * quaternion.x + 2.0 * quaternion.y * quaternion.z)
        + self.z
          * (quaternion.w * quaternion.w
            - quaternion.x * quaternion.x
            - quaternion.y * quaternion.y
            + quaternion.z * quaternion.z),
    };
  }

  #[inline]
  pub fn rotate_by_axis_angle(self, mut axis: Vector3, mut angle: f32) -> Vector3 {
    // Using Euler-Rodrigues Formula
    // Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
    let mut result = self;

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

  #[inline]
  pub fn move_towards(self, target: Vector3, max_distance: f32) -> Vector3 {
    let dx = target.x - self.x;
    let dy = target.y - self.y;
    let dz = target.z - self.z;
    let value = (dx * dx) + (dy * dy) + (dz * dz);

    if value == 0.0 || (max_distance >= 0.0 && value <= max_distance * max_distance) {
      return target;
    }

    let dist = value.sqrt();

    return Vector3 {
      x: self.x + dx / dist * max_distance,
      y: self.y + dy / dist * max_distance,
      z: self.z + dz / dist * max_distance,
    };
  }

  #[inline]
  pub fn lerp(self, other: Vector3, amount: f32) -> Vector3 {
    return Vector3 {
      x: self.x + amount * (other.x - self.x),
      y: self.y + amount * (other.y - self.y),
      z: self.z + amount * (other.z - self.z),
    };
  }

  #[inline]
  pub fn cubic_hermite(
    self,
    tangent1: Vector3,
    v2: Vector3,
    tangent2: Vector3,
    amount: f32,
  ) -> Vector3 {
    let amount_pow2 = amount * amount;
    let amount_pow3 = amount * amount * amount;

    return Vector3 {
      x: (2.0 * amount_pow3 - 3.0 * amount_pow2 + 1.0) * self.x
        + (amount_pow3 - 2.0 * amount_pow2 + amount) * tangent1.x
        + (-2.0 * amount_pow3 + 3.0 * amount_pow2) * v2.x
        + (amount_pow3 - amount_pow2) * tangent2.x,
      y: (2.0 * amount_pow3 - 3.0 * amount_pow2 + 1.0) * self.y
        + (amount_pow3 - 2.0 * amount_pow2 + amount) * tangent1.y
        + (-2.0 * amount_pow3 + 3.0 * amount_pow2) * v2.y
        + (amount_pow3 - amount_pow2) * tangent2.y,
      z: (2.0 * amount_pow3 - 3.0 * amount_pow2 + 1.0) * self.z
        + (amount_pow3 - 2.0 * amount_pow2 + amount) * tangent1.z
        + (-2.0 * amount_pow3 + 3.0 * amount_pow2) * v2.z
        + (amount_pow3 - amount_pow2) * tangent2.z,
    };
  }

  #[inline]
  pub fn reflect(self, normal: Vector3) -> Vector3 {
    // I is the original vector
    // N is the normal of the incident plane
    // R = I - (2*N*(DotProduct[I, N]))

    let dot_product = self.x * normal.x + self.y * normal.y + self.z * normal.z;

    return Vector3 {
      x: self.x - (2.0 * normal.x) * dot_product,
      y: self.y - (2.0 * normal.y) * dot_product,
      z: self.z - (2.0 * normal.z) * dot_product,
    };
  }

  #[inline]
  pub fn min(self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
    };
  }

  #[inline]
  pub fn max(self, other: Vector3) -> Vector3 {
    return Vector3 {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    };
  }

  #[inline]
  pub fn barycenter(self, a: Vector3, b: Vector3, c: Vector3) -> Vector3 {
    let v0 = Vector3 {
      x: b.x - a.x,
      y: b.y - a.y,
      z: b.z - a.z,
    }; // Vector3Subtract(b, a)
    let v1 = Vector3 {
      x: c.x - a.x,
      y: c.y - a.y,
      z: c.z - a.z,
    }; // Vector3Subtract(c, a)
    let v2 = Vector3 {
      x: self.x - a.x,
      y: self.y - a.y,
      z: self.z - a.z,
    }; // Vector3Subtract(p, a)

    let d00 = v0.x * v0.x + v0.y * v0.y + v0.z * v0.z; // Vector3DotProduct(v0, v0)
    let d01 = v0.x * v1.x + v0.y * v1.y + v0.z * v1.z; // Vector3DotProduct(v0, v1)
    let d11 = v1.x * v1.x + v1.y * v1.y + v1.z * v1.z; // Vector3DotProduct(v1, v1)
    let d20 = v2.x * v0.x + v2.y * v0.y + v2.z * v0.z; // Vector3DotProduct(v2, v0)
    let d21 = v2.x * v1.x + v2.y * v1.y + v2.z * v1.z; // Vector3DotProduct(v2, v1)

    let denom = d00 * d11 - d01 * d01;

    return Vector3 {
      y: (d11 * d20 - d01 * d21) / denom,
      z: (d00 * d21 - d01 * d20) / denom,
      x: 1.0 - (d00 * d21 - d01 * d20 + d11 * d20 - d01 * d21) / denom,
    };
  }

  #[inline]
  pub fn unproject(self, projection: Matrix, view: Matrix) -> Vector3 {
    // Calculate unprojected matrix (multiply view matrix by projection matrix) and invert it
    let mat_view_proj = Matrix {
      // MatrixMultiply(view, projection);
      m0: view.m0 * projection.m0
        + view.m1 * projection.m4
        + view.m2 * projection.m8
        + view.m3 * projection.m12,
      m4: view.m0 * projection.m1
        + view.m1 * projection.m5
        + view.m2 * projection.m9
        + view.m3 * projection.m13,
      m8: view.m0 * projection.m2
        + view.m1 * projection.m6
        + view.m2 * projection.m10
        + view.m3 * projection.m14,
      m12: view.m0 * projection.m3
        + view.m1 * projection.m7
        + view.m2 * projection.m11
        + view.m3 * projection.m15,
      m1: view.m4 * projection.m0
        + view.m5 * projection.m4
        + view.m6 * projection.m8
        + view.m7 * projection.m12,
      m5: view.m4 * projection.m1
        + view.m5 * projection.m5
        + view.m6 * projection.m9
        + view.m7 * projection.m13,
      m9: view.m4 * projection.m2
        + view.m5 * projection.m6
        + view.m6 * projection.m10
        + view.m7 * projection.m14,
      m13: view.m4 * projection.m3
        + view.m5 * projection.m7
        + view.m6 * projection.m11
        + view.m7 * projection.m15,
      m2: view.m8 * projection.m0
        + view.m9 * projection.m4
        + view.m10 * projection.m8
        + view.m11 * projection.m12,
      m6: view.m8 * projection.m1
        + view.m9 * projection.m5
        + view.m10 * projection.m9
        + view.m11 * projection.m13,
      m10: view.m8 * projection.m2
        + view.m9 * projection.m6
        + view.m10 * projection.m10
        + view.m11 * projection.m14,
      m14: view.m8 * projection.m3
        + view.m9 * projection.m7
        + view.m10 * projection.m11
        + view.m11 * projection.m15,
      m3: view.m12 * projection.m0
        + view.m13 * projection.m4
        + view.m14 * projection.m8
        + view.m15 * projection.m12,
      m7: view.m12 * projection.m1
        + view.m13 * projection.m5
        + view.m14 * projection.m9
        + view.m15 * projection.m13,
      m11: view.m12 * projection.m2
        + view.m13 * projection.m6
        + view.m14 * projection.m10
        + view.m15 * projection.m14,
      m15: view.m12 * projection.m3
        + view.m13 * projection.m7
        + view.m14 * projection.m11
        + view.m15 * projection.m15,
    };

    // Calculate inverted matrix -> MatrixInvert(matViewProj);
    // Cache the matrix values (speed optimization)
    let a00 = mat_view_proj.m0;
    let a01 = mat_view_proj.m1;
    let a02 = mat_view_proj.m2;
    let a03 = mat_view_proj.m3;
    let a10 = mat_view_proj.m4;
    let a11 = mat_view_proj.m5;
    let a12 = mat_view_proj.m6;
    let a13 = mat_view_proj.m7;
    let a20 = mat_view_proj.m8;
    let a21 = mat_view_proj.m9;
    let a22 = mat_view_proj.m10;
    let a23 = mat_view_proj.m11;
    let a30 = mat_view_proj.m12;
    let a31 = mat_view_proj.m13;
    let a32 = mat_view_proj.m14;
    let a33 = mat_view_proj.m15;

    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    // Calculate the invert determinant (inlined to avoid double-caching)
    let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

    let mat_view_proj_inv = Matrix {
      m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
      m4: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
      m8: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
      m12: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
      m1: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
      m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
      m9: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
      m13: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
      m2: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
      m6: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
      m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
      m14: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
      m3: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
      m7: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
      m11: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
      m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
    };

    // Create quaternion from source point
    let quat = Quaternion {
      x: self.x,
      y: self.y,
      z: self.z,
      w: 1.0,
    };

    // Multiply quat point by unprojecte matrix
    let qtransformed = Quaternion {
      // QuaternionTransform(quat, matViewProjInv)
      x: mat_view_proj_inv.m0 * quat.x
        + mat_view_proj_inv.m4 * quat.y
        + mat_view_proj_inv.m8 * quat.z
        + mat_view_proj_inv.m12 * quat.w,
      y: mat_view_proj_inv.m1 * quat.x
        + mat_view_proj_inv.m5 * quat.y
        + mat_view_proj_inv.m9 * quat.z
        + mat_view_proj_inv.m13 * quat.w,
      z: mat_view_proj_inv.m2 * quat.x
        + mat_view_proj_inv.m6 * quat.y
        + mat_view_proj_inv.m10 * quat.z
        + mat_view_proj_inv.m14 * quat.w,
      w: mat_view_proj_inv.m3 * quat.x
        + mat_view_proj_inv.m7 * quat.y
        + mat_view_proj_inv.m11 * quat.z
        + mat_view_proj_inv.m15 * quat.w,
    };

    // Normalized world points in vectors
    return Vector3 {
      x: qtransformed.x / qtransformed.w,
      y: qtransformed.y / qtransformed.w,
      z: qtransformed.z / qtransformed.w,
    };
  }

  #[inline]
  pub fn to_array(self) -> [f32; 3] {
    return [self.x, self.y, self.z];
  }

  #[inline]
  pub fn invert(self) -> Vector3 {
    return Vector3 {
      x: 1.0 / self.x,
      y: 1.0 / self.y,
      z: 1.0 / self.z,
    };
  }

  #[inline]
  pub fn clamp(self, min: Vector3, max: Vector3) -> Vector3 {
    return Vector3 {
      x: max.x.min(min.x.max(self.x)),
      y: max.y.min(min.y.max(self.y)),
      z: max.z.min(min.z.max(self.z)),
    };
  }

  #[inline]
  pub fn clamp_value(self, min: f32, max: f32) -> Vector3 {
    let mut length = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);

    if length > 0.0 {
      length = length.sqrt();

      let scale = if length < min {
        min / length
      } else if length > max {
        max / length
      } else {
        1.0
      };

      return Vector3 {
        x: self.x * scale,
        y: self.y * scale,
        z: self.z * scale,
      };
    }

    return self;
  }

  #[inline]
  pub fn refract(self, normal: Vector3, ratio: f32) -> Vector3 {
    let dot = self.x * normal.x + self.y * normal.y + self.z * normal.z;
    let mut d = 1.0 - ratio * ratio * (1.0 - dot * dot);

    if d >= 0.0 {
      d = d.sqrt();

      return Vector3 {
        x: ratio * self.x - (ratio * dot + d) * normal.x,
        y: ratio * self.y - (ratio * dot + d) * normal.y,
        z: ratio * self.z - (ratio * dot + d) * normal.z,
      };
    }

    return Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    };
  }

  // Calculate quaternion based on the rotation from one vector to another
  #[inline]
  pub fn to_quaternion(self, to: Vector3) -> Quaternion {
    let cos2_theta = self.x * to.x + self.y * to.y + self.z * to.z; // Vector3DotProduct(from, to)
    let cross = Vector3 {
      x: self.y * to.z - self.z * to.y,
      y: self.z * to.x - self.x * to.z,
      z: self.x * to.y - self.y * to.x,
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

  // Get rotation quaternion for an angle and axis
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn from_axis_angle_to_quaternion(mut self, mut angle: f32) -> Quaternion {
    let axis_length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

    if axis_length != 0.0 {
      angle *= 0.5;

      // Vector3Normalize(axis)
      let length = axis_length;
      let ilength = if length == 0.0 { 1.0 } else { 1.0 / length };
      self.x *= ilength;
      self.y *= ilength;
      self.z *= ilength;

      let sinres = angle.sin();
      let cosres = angle.cos();

      let q = Quaternion {
        x: self.x * sinres,
        y: self.y * sinres,
        z: self.z * sinres,
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

  // Get xyz-rotation matrix
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn rotate_xyz(self) -> Matrix {
    let (cos_x, cos_y, cos_z) = ((-self.x).cos(), (-self.y).cos(), (-self.z).cos());
    let (sin_x, sin_y, sin_z) = ((-self.x).sin(), (-self.y).sin(), (-self.z).sin());

    return Matrix {
      m0: cos_z * cos_y,
      m4: sin_z * cos_y,
      m8: -sin_y,
      m12: 0.0,
      m1: (cos_z * sin_y * sin_x) - (sin_z * cos_x),
      m5: (sin_z * sin_y * sin_x) + (cos_z * cos_x),
      m9: cos_y * sin_x,
      m13: 0.0,
      m2: (cos_z * sin_y * cos_x) + (sin_z * sin_x),
      m6: (sin_z * sin_y * cos_x) - (cos_z * sin_x),
      m10: cos_y * cos_x,
      m14: 0.0,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
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

impl AddAssign for Vector3 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
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

impl SubAssign for Vector3 {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
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

impl Mul<Matrix> for Vector3 {
  type Output = Self;

  fn mul(self, rhs: Matrix) -> Self {
    return Vector3 {
      x: rhs.m0 * self.x + rhs.m4 * self.y + rhs.m8 * self.z + rhs.m12,
      y: rhs.m1 * self.x + rhs.m5 * self.y + rhs.m9 * self.z + rhs.m13,
      z: rhs.m2 * self.x + rhs.m6 * self.y + rhs.m10 * self.z + rhs.m14,
    };
  }
}

impl MulAssign for Vector3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x;
    self.y *= rhs.y;
  }
}

impl MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
  }
}

impl MulAssign<Matrix> for Vector3 {
  fn mul_assign(&mut self, rhs: Matrix) {
    self.x = rhs.m0 * self.x + rhs.m4 * self.y + rhs.m8 * self.z + rhs.m12;
    self.y = rhs.m1 * self.x + rhs.m5 * self.y + rhs.m9 * self.z + rhs.m13;
    self.z = rhs.m2 * self.x + rhs.m6 * self.y + rhs.m10 * self.z + rhs.m14;
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

impl Div for Vector3 {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    return Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
    };
  }
}

impl Div<f32> for Vector3 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self {
    return Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    };
  }
}

impl DivAssign for Vector3 {
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x;
    self.y /= rhs.y;
    self.z /= rhs.z;
  }
}

impl DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, rhs: f32) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
  }
}
