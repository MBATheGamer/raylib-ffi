#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn dot_product(&self, other: Vector3) -> f32 {
    return self.x * other.x + self.y * other.y + self.z * other.z;
  }

  pub fn length(&self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z;
  }

  pub fn lerp(&self, other: Vector3, amount: f32) -> Vector3 {
    return Vector3 {
      x: self.x + amount * (other.x - self.x),
      y: self.y + amount * (other.y - self.y),
      z: self.z + amount * (other.z - self.z),
    };
  }
}
