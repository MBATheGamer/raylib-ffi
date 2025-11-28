#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Font {
  pub base_size: i32,                // Base size (default chars height)
  pub glyph_count: i32,              // Number of glyph characters
  pub glyph_padding: i32,            // Padding around the glyph characters
  pub texture: super::Texture,       // Texture atlas containing the glyphs
  pub recs: *mut super::Rectangle,   // Rectangles in texture for the glyphs
  pub glyphs: *mut super::GlyphInfo, // Glyphs info data
}
