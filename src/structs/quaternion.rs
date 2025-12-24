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
}
