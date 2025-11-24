#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Font {
  base_size: i32,         // Base size (default chars height)
  glyph_count: i32,       // Number of glyph characters
  glyph_padding: i32,     // Padding around the glyph characters
  texture: Texture,       // Texture atlas containing the glyphs
  recs: *mut Rectangle,   // Rectangles in texture for the glyphs
  glyphs: *mut GlyphInfo, // Glyphs info data
}
