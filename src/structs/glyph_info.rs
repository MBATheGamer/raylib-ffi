#[repr(C)]
#[derive(Clone, Copy)]
pub struct GlyphInfo {
  value: i32,          // Character value (Unicode)
  offset_x: i32,       // Character offset X when drawing
  offset_y: i32,       // Character offset Y when drawing
  advance_x: i32,      // Character advance position X
  image: super::Image, // Character image data
}
