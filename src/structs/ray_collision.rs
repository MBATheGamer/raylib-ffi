#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RayCollision {
  pub hit: bool,              // Did the ray hit something?
  pub distance: f32,          // Distance to the nearest hit
  pub point: super::Vector3,  // Point of the nearest hit
  pub normal: super::Vector3, // Surface normal of hit
}
