use crate::structs::{Camera2D, Color, RenderTexture2D, Vector2};

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
  pub unsafe fn WindowShouldClose() -> bool;
  pub unsafe fn GetScreenWidth() -> i32;

  // Cursor-related functions
  pub unsafe fn ShowCursor();
  pub unsafe fn HideCursor();
  pub unsafe fn IsCursorHidden() -> bool;

  // Drawing-related funtions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();
  pub unsafe fn EndDrawing();
  pub unsafe fn BeginMode2D(camera: Camera2D);
  pub unsafe fn EndMode2D();
  pub unsafe fn BeginTextureMode(target: RenderTexture2D);

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);
  pub unsafe fn GetFrameTime() -> f32;
  pub unsafe fn GetFPS() -> i32;

  // Random values generation functions
  pub unsafe fn GetRandomValue(min: i32, max: i32) -> i32;

  // Misc. functions
  pub unsafe fn SetConfigFlags(flags: u32);

  // Input-related functions: Keyboard
  pub unsafe fn IsKeyPressed(key: i32) -> bool;
  pub unsafe fn IsKeyDown(key: i32) -> bool;

  // Input-related functions: Gamepads
  pub unsafe fn IsGamepadAvailable(gamepad: i32) -> bool;
  pub unsafe fn GetGamepadName(gamepad: i32) -> *const i8;
  pub unsafe fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool;
  pub unsafe fn GetGamepadButtonPressed() -> i32;
  pub unsafe fn GetGamepadAxisCount(gamepad: i32) -> i32;
  pub unsafe fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32;

  // Input-related functions: Mouse
  pub unsafe fn IsMouseButtonPressed(button: i32) -> bool;
  pub unsafe fn IsMouseButtonDown(button: i32) -> bool;
  pub unsafe fn IsMouseButtonReleased(button: i32) -> bool;
  pub unsafe fn GetMousePosition() -> Vector2;
  pub unsafe fn GetMouseWheelMove() -> f32;

  // Input-related functions: Touch
  pub unsafe fn GetTouchPosition(index: i32) -> Vector2;
  pub unsafe fn GetTouchPointCount() -> i32;

  // Gestures and touch handling functions
  pub unsafe fn GetGestureDetected() -> i32;
  pub unsafe fn GetGestureDragAngle() -> f32;
  pub unsafe fn GetGesturePinchAngle() -> f32;
}
