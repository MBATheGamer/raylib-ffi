use crate::enums::CameraProjection;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

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
pub struct Rectangle {
  pub x: f32,      // Rectangle top-left corner position x
  pub y: f32,      // Rectangle top-left corner position y
  pub width: f32,  // Rectangle width
  pub height: f32, // Rectangle height
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Texture {
  pub id: u32,      // OpenGL texture id
  pub width: i32,   // Texture base width
  pub height: i32,  // Texture base height
  pub mipmaps: i32, // Mipmap levels, 1 by default
  pub format: i32,  // Data format (PixelFormat type)
}

pub type Texture2D = Texture;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RenderTexture {
  pub id: u32,          // OpenGL framebuffer object id
  pub texture: Texture, // Color buffer attachment texture
  pub depth: Texture,   // Depth buffer attachment texture
}

pub type RenderTexture2D = RenderTexture;

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
#[derive(Clone, Copy, Default, Debug)]
pub struct FilePathList {
  pub capacity: u32,
  pub count: u32,
  pub paths: *mut *mut i8,
}
