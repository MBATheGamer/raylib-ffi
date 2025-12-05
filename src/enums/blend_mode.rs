#[repr(i32)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum BlendMode {
  Alpha = 0,        // Blend textures considering alpha (default)
  Additive,         // Blend textures adding colors
  Multiplied,       // Blend textures multiplying colors
  AddColors,        // Blend textures adding colors (alternative)
  SubtractColors,   // Blend textures subtracting colors (alternative)
  AlphaPremultiply, // Blend premultiplied textures considering alpha
  Custom,           // Blend textures using custom src/dst factors (use rlSetBlendFactors())
  CustomSeparate, // Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
}
