mod color;
mod font;
mod glyph_info;
mod image;
mod matrix;
mod rectangle;
mod render_texture;
mod texture;
mod vector2;
mod vector3;

pub use color::Color;
pub use font::Font;
pub use glyph_info::GlyphInfo;
pub use image::Image;
pub use matrix::Matrix;
pub use rectangle::Rectangle;
pub use render_texture::RenderTexture;
pub use texture::Texture;
pub use vector2::Vector2;
pub use vector3::Vector3;

use crate::enums::CameraProjection;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Camera2D {
  pub offset: Vector2, // Camera offset (displacement from target)
  pub target: Vector2, // Camera target (rotation and zoom origin)
  pub rotation: f32,   // Camera rotation in degrees
  pub zoom: f32,       // Camera zoom (scaling), should be 1.0f by default
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Camera3D {
  pub position: Vector3,            // Camera position
  pub target: Vector3,              // Camera target it looks-at
  pub up: Vector3,                  // Camera up vector (rotation over its axis)
  pub fovy: f32, // Camera field-of-view aperture in Y (degrees) in perspective, used as near plane height in world units in orthographic
  pub projection: CameraProjection, // Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Shader {
  pub id: u32,        // Shader program id
  pub locs: *mut i32, // Shader locations array (RL_MAX_SHADER_LOCATIONS)
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Ray {
  pub position: Vector3,  // Ray position (origin)
  pub direction: Vector3, // Ray direction (normalized)
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RayCollision {
  pub hit: bool,       // Did the ray hit something?
  pub distance: f32,   // Distance to the nearest hit
  pub point: Vector3,  // Point of the nearest hit
  pub normal: Vector3, // Surface normal of hit
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BoundingBox {
  pub min: Vector3, // Minimum vertex box-corner
  pub max: Vector3, // Maximum vertex box-corner
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VrDeviceInfo {
  pub h_resolution: i32,                // Horizontal resolution in pixels
  pub v_resolution: i32,                // Vertical resolution in pixels
  pub h_screen_size: f32,               // Horizontal size in meters
  pub v_screen_size: f32,               // Vertical size in meters
  pub eye_to_screen_distance: f32,      // Distance between eye and display in meters
  pub lens_separation_distance: f32,    // Lens separation distance in meters
  pub interpupillary_distance: f32,     // IPD (distance between pupils) in meters
  pub lens_distortion_values: [f32; 4], // Lens distortion constant parameters
  pub chroma_ab_correction: [f32; 4],   // Chromatic aberration correction parameters
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VrStereoConfig {
  pub projection: [Matrix; 2],       // VR projection matrices (per eye)
  pub view_offset: [Matrix; 2],      // VR view offset matrices (per eye)
  pub left_lens_center: [f32; 2],    // VR left lens center
  pub right_lens_center: [f32; 2],   // VR right lens center
  pub left_screen_center: [f32; 2],  // VR left screen center
  pub right_screen_center: [f32; 2], // VR right screen center
  pub scale: [f32; 2],               // VR distortion scale
  pub scale_in: [f32; 2],            // VR distortion scale in
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FilePathList {
  pub capacity: u32,
  pub count: u32,
  pub paths: *mut *mut i8,
}
