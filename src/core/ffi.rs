unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);
}
