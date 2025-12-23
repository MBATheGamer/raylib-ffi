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

  #[inline]
  pub fn one() -> Vector4 {
    return Vector4 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
      w: 1.0,
    };
  }

  #[inline]
  pub fn add(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x + v2.x,
      y: self.y + v2.y,
      z: self.z + v2.z,
      w: self.w + v2.w,
    };
  }

  #[inline]
  pub fn add_value(self, add: f32) -> Vector4 {
    return Vector4 {
      x: self.x + add,
      y: self.y + add,
      z: self.z + add,
      w: self.w + add,
    };
  }

  #[inline]
  pub fn subtract(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x - v2.x,
      y: self.y - v2.y,
      z: self.z - v2.z,
      w: self.w - v2.w,
    };
  }

  #[inline]
  pub fn subtract_value(self, add: f32) -> Vector4 {
    return Vector4 {
      x: self.x - add,
      y: self.y - add,
      z: self.z - add,
      w: self.w - add,
    };
  }

  #[inline]
  pub fn length(self) -> f32 {
    return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
  }

  #[inline]
  pub fn length_sqr(self) -> f32 {
    return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
  }

  #[inline]
  pub fn dot_product(self, v2: Vector4) -> f32 {
    return self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w;
  }

  // Calculate distance between two vectors
  #[inline]
  pub fn distance(self, v2: Vector4) -> f32 {
    return ((self.x - v2.x) * (self.x - v2.x)
      + (self.y - v2.y) * (self.y - v2.y)
      + (self.z - v2.z) * (self.z - v2.z)
      + (self.w - v2.w) * (self.w - v2.w))
      .sqrt();
  }

  // Calculate square distance between two vectors
  #[inline]
  pub fn distance_sqr(self, v2: Vector4) -> f32 {
    return (self.x - v2.x) * (self.x - v2.x)
      + (self.y - v2.y) * (self.y - v2.y)
      + (self.z - v2.z) * (self.z - v2.z)
      + (self.w - v2.w) * (self.w - v2.w);
  }

  #[inline]
  pub fn scale(self, scale: f32) -> Vector4 {
    return Vector4 {
      x: self.x * scale,
      y: self.y * scale,
      z: self.z * scale,
      w: self.w * scale,
    };
  }

  // Multiply vector by vector
  #[inline]
  pub fn multiply(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x * v2.x,
      y: self.y * v2.y,
      z: self.z * v2.z,
      w: self.w * v2.w,
    };
  }

  // Negate vector
  #[inline]
  pub fn negate(self) -> Vector4 {
    return Vector4 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    };
  }

  // Divide vector by vector
  #[inline]
  pub fn divide(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x / v2.x,
      y: self.y / v2.y,
      z: self.z / v2.z,
      w: self.w / v2.w,
    };
  }

  // Normalize provided vector
  #[inline]
  pub fn normalize(self) -> Vector4 {
    let length = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();

    if length > 0.0 {
      let length = 1.0 / length;
      return Vector4 {
        x: self.x * length,
        y: self.y * length,
        z: self.z * length,
        w: self.w * length,
      };
    }

    return Vector4 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 0.0,
    };
  }

  // Get min value for each pair of components
  #[inline]
  pub fn min(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x.min(v2.x),
      y: self.y.min(v2.y),
      z: self.z.min(v2.z),
      w: self.w.min(v2.w),
    };
  }

  // Get max value for each pair of components
  #[inline]
  pub fn max(self, v2: Vector4) -> Vector4 {
    return Vector4 {
      x: self.x.max(v2.x),
      y: self.y.max(v2.y),
      z: self.z.max(v2.z),
      w: self.w.max(v2.w),
    };
  }

  // Calculate linear interpolation between two vectors
  #[inline]
  pub fn lerp(self, v2: Vector4, amount: f32) -> Vector4 {
    return Vector4 {
      x: self.x + amount * (v2.x - self.x),
      y: self.y + amount * (v2.y - self.y),
      z: self.z + amount * (v2.z - self.z),
      w: self.w + amount * (v2.w - self.w),
    };
  }

  // Move Vector towards target
  #[inline]
  pub fn move_towards(self, target: Vector4, max_distance: f32) -> Vector4 {
    let dx = target.x - self.x;
    let dy = target.y - self.y;
    let dz = target.z - self.z;
    let dw = target.w - self.w;
    let value = (dx * dx) + (dy * dy) + (dz * dz) + (dw * dw);

    if value == 0.0 || (max_distance >= 0.0 && value <= max_distance * max_distance) {
      return target;
    }

    let dist = value.sqrt();

    return Vector4 {
      x: self.x + dx / dist * max_distance,
      y: self.y + dy / dist * max_distance,
      z: self.z + dz / dist * max_distance,
      w: self.w + dw / dist * max_distance,
    };
  }
}
