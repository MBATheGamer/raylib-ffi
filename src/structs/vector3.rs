#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn lerp(&self, other_vector: Vector3, amount: f32) -> Vector3 {
    return Vector3 {
      x: self.x + amount * (other_vector.x - self.x),
      y: self.y + amount * (other_vector.y - self.y),
      z: self.z + amount * (other_vector.z - self.z),
    };
  }
}
