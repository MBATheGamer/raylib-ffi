#[repr(i32)]
#[derive(Clone, Copy)]
pub enum CameraProjection {
  Perspective = 0, // Perspective projection
  Orthographic,    // Orthographic projection
}

impl Default for CameraProjection {
  fn default() -> Self {
    return CameraProjection::Perspective;
  }
}
