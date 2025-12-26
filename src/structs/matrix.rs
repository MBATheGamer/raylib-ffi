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
}
