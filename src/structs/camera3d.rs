use crate::enums::CameraProjection;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Camera3D {
  pub position: super::Vector3,     // Camera position
  pub target: super::Vector3,       // Camera target it looks-at
  pub up: super::Vector3,           // Camera up vector (rotation over its axis)
  pub fovy: f32, // Camera field-of-view aperture in Y (degrees) in perspective, used as near plane height in world units in orthographic
  pub projection: CameraProjection, // Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
}
