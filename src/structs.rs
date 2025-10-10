#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}
