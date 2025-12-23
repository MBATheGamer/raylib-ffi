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
}
