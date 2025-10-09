use crate::structs::Color;

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn WindowShouldClose() -> bool;

  // Drawing-related funtions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);
}
