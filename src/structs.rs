#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
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
#[derive(Default)]
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
