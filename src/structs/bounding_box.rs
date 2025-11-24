#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BoundingBox {
  pub min: super::Vector3, // Minimum vertex box-corner
  pub max: super::Vector3, // Maximum vertex box-corner
}
