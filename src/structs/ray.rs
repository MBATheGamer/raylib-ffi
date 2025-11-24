#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Ray {
  pub position: super::Vector3,  // Ray position (origin)
  pub direction: super::Vector3, // Ray direction (normalized)
}
