#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Camera2D {
  pub offset: super::Vector2, // Camera offset (displacement from target)
  pub target: super::Vector2, // Camera target (rotation and zoom origin)
  pub rotation: f32,          // Camera rotation in degrees
  pub zoom: f32,              // Camera zoom (scaling), should be 1.0f by default
}
