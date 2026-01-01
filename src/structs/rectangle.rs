#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Rectangle {
  pub x: f32,      // Rectangle top-left corner position x
  pub y: f32,      // Rectangle top-left corner position y
  pub width: f32,  // Rectangle width
  pub height: f32, // Rectangle height
}

impl Rectangle {
  #[inline]
  pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    return Rectangle {
      x,
      y,
      width,
      height,
    };
  }
}
