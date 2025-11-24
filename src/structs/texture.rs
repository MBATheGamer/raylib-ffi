#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Texture {
  pub id: u32,      // OpenGL texture id
  pub width: i32,   // Texture base width
  pub height: i32,  // Texture base height
  pub mipmaps: i32, // Mipmap levels, 1 by default
  pub format: i32,  // Data format (PixelFormat type)
}
