pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
  return start + amount * (end - start);
}

pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
  return (value - start) / (end - start);
}

pub fn remap(
  value: f32,
  input_start: f32,
  input_end: f32,
  output_start: f32,
  output_end: f32,
) -> f32 {
  return (value - input_start) / (input_end - input_start) * (output_end - output_start)
    + output_start;
}
