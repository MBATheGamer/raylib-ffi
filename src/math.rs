pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
  return start + amount * (end - start);
}

pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
  return (value - start) / (end - start);
}
