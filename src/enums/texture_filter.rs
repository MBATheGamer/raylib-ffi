#[repr(i32)]
pub enum TextureFilter {
  TextureFilterPoint = 0,      // No filter, just pixel approximation
  TextureFilterBilinear,       // Linear filtering
  TextureFilterTrilinear,      // Trilinear filtering (linear with mipmaps)
  TextureFilterAnisotropic4x,  // Anisotropic filtering 4x
  TextureFilterAnisotropic8x,  // Anisotropic filtering 8x
  TextureFilterAnisotropic16x, // Anisotropic filtering 16x
}
