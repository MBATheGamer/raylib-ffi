#[repr(i32)]
pub enum TextureFilter {
  Point = 0,      // No filter, just pixel approximation
  Bilinear,       // Linear filtering
  Trilinear,      // Trilinear filtering (linear with mipmaps)
  Anisotropic4x,  // Anisotropic filtering 4x
  Anisotropic8x,  // Anisotropic filtering 8x
  Anisotropic16x, // Anisotropic filtering 16x
}
