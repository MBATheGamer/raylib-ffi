#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn angle(&self, other: Vector3) -> f32 {
    let cross = Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    };
    let len = (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt();
    let dot = self.x * other.x + self.y * other.y + self.z * other.z;

    return len.atan2(dot);
  }

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

  pub fn rotate_by_axis_angle(&self, mut axis: Vector3, mut angle: f32) -> Vector3 {
    // Using Euler-Rodrigues Formula
    // Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
    let mut result = *self;

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
}
