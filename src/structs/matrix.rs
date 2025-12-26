#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Matrix {
  pub m0: f32,
  pub m4: f32,
  pub m8: f32,
  pub m12: f32, // Matrix first row (4 components)
  pub m1: f32,
  pub m5: f32,
  pub m9: f32,
  pub m13: f32, // Matrix second row (4 components)
  pub m2: f32,
  pub m6: f32,
  pub m10: f32,
  pub m14: f32, // Matrix third row (4 components)
  pub m3: f32,
  pub m7: f32,
  pub m11: f32,
  pub m15: f32, // Matrix fourth row (4 components)
}

impl Matrix {
  // Compute matrix determinant
  #[inline]
  pub fn determinant(self) -> f32 {
    // Cache the matrix values (speed optimization)
    let (a00, a01, a02, a03) = (self.m0, self.m1, self.m2, self.m3);
    let (a10, a11, a12, a13) = (self.m4, self.m5, self.m6, self.m7);
    let (a20, a21, a22, a23) = (self.m8, self.m9, self.m10, self.m11);
    let (a30, a31, a32, a33) = (self.m12, self.m13, self.m14, self.m15);

    return a30 * a21 * a12 * a03 - a20 * a31 * a12 * a03 - a30 * a11 * a22 * a03
      + a10 * a31 * a22 * a03
      + a20 * a11 * a32 * a03
      - a10 * a21 * a32 * a03
      - a30 * a21 * a02 * a13
      + a20 * a31 * a02 * a13
      + a30 * a01 * a22 * a13
      - a00 * a31 * a22 * a13
      - a20 * a01 * a32 * a13
      + a00 * a21 * a32 * a13
      + a30 * a11 * a02 * a23
      - a10 * a31 * a02 * a23
      - a30 * a01 * a12 * a23
      + a00 * a31 * a12 * a23
      + a10 * a01 * a32 * a23
      - a00 * a11 * a32 * a23
      - a20 * a11 * a02 * a33
      + a10 * a21 * a02 * a33
      + a20 * a01 * a12 * a33
      - a00 * a21 * a12 * a33
      - a10 * a01 * a22 * a33
      + a00 * a11 * a22 * a33;
  }

  // Get the trace of the matrix (sum of the values along the diagonal)
  #[inline]
  pub fn trace(self) -> f32 {
    return self.m0 + self.m5 + self.m10 + self.m15;
  }

  // Transposes provided matrix
  #[inline]
  pub fn transpose(self) -> Matrix {
    return Matrix {
      m0: self.m0,
      m1: self.m4,
      m2: self.m8,
      m3: self.m12,
      m4: self.m1,
      m5: self.m5,
      m6: self.m9,
      m7: self.m13,
      m8: self.m2,
      m9: self.m6,
      m10: self.m10,
      m11: self.m14,
      m12: self.m3,
      m13: self.m7,
      m14: self.m11,
      m15: self.m15,
    };
  }

  // Invert provided matrix
  #[inline]
  pub fn invert(self) -> Matrix {
    // Cache the matrix values (speed optimization)
    let (a00, a01, a02, a03) = (self.m0, self.m1, self.m2, self.m3);
    let (a10, a11, a12, a13) = (self.m4, self.m5, self.m6, self.m7);
    let (a20, a21, a22, a23) = (self.m8, self.m9, self.m10, self.m11);
    let (a30, a31, a32, a33) = (self.m12, self.m13, self.m14, self.m15);

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

    return Matrix {
      m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
      m1: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
      m2: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
      m3: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
      m4: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
      m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
      m6: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
      m7: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
      m8: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
      m9: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
      m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
      m11: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
      m12: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
      m13: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
      m14: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
      m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
    };
  }

  // Get identity matrix
  #[inline]
  pub fn identity() -> Matrix {
    return Matrix {
      m0: 1.0,
      m4: 0.0,
      m8: 0.0,
      m12: 0.0,
      m1: 0.0,
      m5: 1.0,
      m9: 0.0,
      m13: 0.0,
      m2: 0.0,
      m6: 0.0,
      m10: 1.0,
      m14: 0.0,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
  }

  // Add two matrices
  #[inline]
  pub fn add(self, right: Matrix) -> Matrix {
    return Matrix {
      m0: self.m0 + right.m0,
      m1: self.m1 + right.m1,
      m2: self.m2 + right.m2,
      m3: self.m3 + right.m3,
      m4: self.m4 + right.m4,
      m5: self.m5 + right.m5,
      m6: self.m6 + right.m6,
      m7: self.m7 + right.m7,
      m8: self.m8 + right.m8,
      m9: self.m9 + right.m9,
      m10: self.m10 + right.m10,
      m11: self.m11 + right.m11,
      m12: self.m12 + right.m12,
      m13: self.m13 + right.m13,
      m14: self.m14 + right.m14,
      m15: self.m15 + right.m15,
    };
  }

  // Subtract two matrices (left - right)
  #[inline]
  pub fn subtract(self, right: Matrix) -> Matrix {
    return Matrix {
      m0: self.m0 - right.m0,
      m1: self.m1 - right.m1,
      m2: self.m2 - right.m2,
      m3: self.m3 - right.m3,
      m4: self.m4 - right.m4,
      m5: self.m5 - right.m5,
      m6: self.m6 - right.m6,
      m7: self.m7 - right.m7,
      m8: self.m8 - right.m8,
      m9: self.m9 - right.m9,
      m10: self.m10 - right.m10,
      m11: self.m11 - right.m11,
      m12: self.m12 - right.m12,
      m13: self.m13 - right.m13,
      m14: self.m14 - right.m14,
      m15: self.m15 - right.m15,
    };
  }

  // Get two matrix multiplication
  // NOTE: When multiplying matrices... the order matters!
  #[inline]
  pub fn multiply(self, right: Matrix) -> Matrix {
    return Matrix {
      m0: self.m0 * right.m0 + self.m1 * right.m4 + self.m2 * right.m8 + self.m3 * right.m12,
      m1: self.m0 * right.m1 + self.m1 * right.m5 + self.m2 * right.m9 + self.m3 * right.m13,
      m2: self.m0 * right.m2 + self.m1 * right.m6 + self.m2 * right.m10 + self.m3 * right.m14,
      m3: self.m0 * right.m3 + self.m1 * right.m7 + self.m2 * right.m11 + self.m3 * right.m15,
      m4: self.m4 * right.m0 + self.m5 * right.m4 + self.m6 * right.m8 + self.m7 * right.m12,
      m5: self.m4 * right.m1 + self.m5 * right.m5 + self.m6 * right.m9 + self.m7 * right.m13,
      m6: self.m4 * right.m2 + self.m5 * right.m6 + self.m6 * right.m10 + self.m7 * right.m14,
      m7: self.m4 * right.m3 + self.m5 * right.m7 + self.m6 * right.m11 + self.m7 * right.m15,
      m8: self.m8 * right.m0 + self.m9 * right.m4 + self.m10 * right.m8 + self.m11 * right.m12,
      m9: self.m8 * right.m1 + self.m9 * right.m5 + self.m10 * right.m9 + self.m11 * right.m13,
      m10: self.m8 * right.m2 + self.m9 * right.m6 + self.m10 * right.m10 + self.m11 * right.m14,
      m11: self.m8 * right.m3 + self.m9 * right.m7 + self.m10 * right.m11 + self.m11 * right.m15,
      m12: self.m12 * right.m0 + self.m13 * right.m4 + self.m14 * right.m8 + self.m15 * right.m12,
      m13: self.m12 * right.m1 + self.m13 * right.m5 + self.m14 * right.m9 + self.m15 * right.m13,
      m14: self.m12 * right.m2 + self.m13 * right.m6 + self.m14 * right.m10 + self.m15 * right.m14,
      m15: self.m12 * right.m3 + self.m13 * right.m7 + self.m14 * right.m11 + self.m15 * right.m15,
    };
  }

  // Get translation matrix
  #[inline]
  pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
    return Matrix {
      m0: 1.0,
      m4: 0.0,
      m8: 0.0,
      m12: x,
      m1: 0.0,
      m5: 1.0,
      m9: 0.0,
      m13: y,
      m2: 0.0,
      m6: 0.0,
      m10: 1.0,
      m14: z,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
  }

  // Get x-rotation matrix
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn rotate_x(angle: f32) -> Matrix {
    let cos_res = angle.cos();
    let sin_res = angle.sin();

    return Matrix {
      m0: 1.0,
      m4: 0.0,
      m8: 0.0,
      m12: 0.0,
      m1: 0.0,
      m5: cos_res,
      m9: -sin_res,
      m13: 0.0,
      m2: 0.0,
      m6: sin_res,
      m10: cos_res,
      m14: 0.0,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
  }

  // Get y-rotation matrix
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn rotate_y(angle: f32) -> Matrix {
    let cos_res = angle.cos();
    let sin_res = angle.sin();

    return Matrix {
      m0: cos_res,
      m4: 0.0,
      m8: sin_res,
      m12: 0.0,
      m1: 0.0,
      m5: 1.0,
      m9: 0.0,
      m13: 0.0,
      m2: -sin_res,
      m6: 0.0,
      m10: cos_res,
      m14: 0.0,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
  }

  // Get z-rotation matrix
  // NOTE: Angle must be provided in radians
  #[inline]
  pub fn rotate_z(angle: f32) -> Matrix {
    let cos_res = angle.cos();
    let sin_res = angle.sin();

    return Matrix {
      m0: cos_res,
      m4: -sin_res,
      m8: 0.0,
      m12: 0.0,
      m1: sin_res,
      m5: cos_res,
      m9: 0.0,
      m13: 0.0,
      m2: 0.0,
      m6: 0.0,
      m10: 1.0,
      m14: 0.0,
      m3: 0.0,
      m7: 0.0,
      m11: 0.0,
      m15: 1.0,
    };
  }
}
