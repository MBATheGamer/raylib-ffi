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
