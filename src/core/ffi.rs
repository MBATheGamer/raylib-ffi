use crate::structs::Color;

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
  pub unsafe fn WindowShouldClose() -> bool;

  // Cursor-related functions
  pub unsafe fn ShowCursor();
  pub unsafe fn HideCursor();
  pub unsafe fn IsCursorHidden() -> bool;

  // Drawing-related funtions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();
  pub unsafe fn EndDrawing();

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);
  pub unsafe fn GetFrameTime() -> f32;
  pub unsafe fn GetFPS() -> i32;

  // Input-related functions: Keyboard
  pub unsafe fn IsKeyPressed(key: i32) -> bool;
  pub unsafe fn IsKeyDown(key: i32) -> bool;

  // Input-related functions: Mouse
  pub unsafe fn GetMouseWheelMove() -> f32;
}
