#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RenderTexture {
  pub id: u32,                 // OpenGL framebuffer object id
  pub texture: super::Texture, // Color buffer attachment texture
  pub depth: super::Texture,   // Depth buffer attachment texture
}
