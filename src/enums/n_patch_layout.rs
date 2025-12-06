#[repr(i32)]
#[derive(Clone, Copy)]
pub enum NPatchLayout {
  NinePatch = 0,        // Npatch layout: 3x3 tiles
  ThreePatchVertical,   // Npatch layout: 1x3 tiles
  ThreePatchHorizontal, // Npatch layout: 3x1 tiles
}
