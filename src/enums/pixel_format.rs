#[repr(i32)]
#[derive(Clone, Copy)]
pub enum PixelFormat {
  UncompressedGrayscale = 1, // 8 bit per pixel (no alpha)
  UncompressedGrayAlpha,     // 8*2 bpp (2 channels)
  UncompressedR5G6B5,        // 16 bpp
  UncompressedR8G8B8,        // 24 bpp
  UncompressedR5g5B5A1,      // 16 bpp (1 bit alpha)
  UncompressedR4G4B4A4,      // 16 bpp (4 bit alpha)
  UncompressedR8G8B8A8,      // 32 bpp
  UncompressedR32,           // 32 bpp (1 channel - float)
  UncompressedR32G32B32,     // 32*3 bpp (3 channels - float)
  UncompressedR32G32B32A32,  // 32*4 bpp (4 channels - float)
  UncompressedR16,           // 16 bpp (1 channel - half float)
  UncompressedR16G16B16,     // 16*3 bpp (3 channels - half float)
  UncompressedR16G16B16A16,  // 16*4 bpp (4 channels - half float)
  CompressedDxt1RGB,         // 4 bpp (no alpha)
  CompressedDxt1RGBA,        // 4 bpp (1 bit alpha)
  CompressedDxt3RGBA,        // 8 bpp
  CompressedDxt5RGBA,        // 8 bpp
  CompressedEtc1RGB,         // 4 bpp
  CompressedEtc2RGB,         // 4 bpp
  CompressedEtc2EacRGBA,     // 8 bpp
  CompressedPvrtRGB,         // 4 bpp
  CompressedPvrtRGBA,        // 4 bpp
  CompressedAstc4x4RGBA,     // 8 bpp
  CompressedAstc8x8RGBA,     // 2 bpp
}
