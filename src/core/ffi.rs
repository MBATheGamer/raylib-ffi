use std::ffi::c_void;

use crate::structs::{
  Camera2D, Camera3D, Color, FilePathList, Ray, RenderTexture2D, Shader, Vector2, Vector3,
  VrDeviceInfo, VrStereoConfig,
};

unsafe extern "C" {
  // Window-related functions
  pub unsafe fn InitWindow(width: i32, height: i32, title: *const i8);
  pub unsafe fn CloseWindow();
  pub unsafe fn WindowShouldClose() -> bool;
  pub unsafe fn IsWindowState(flag: u32) -> bool;
  pub unsafe fn SetWindowState(flags: u32);
  pub unsafe fn ClearWindowState(flags: u32);
  pub unsafe fn ToggleFullscreen();
  pub unsafe fn ToggleBorderlessWindowed();
  pub unsafe fn MaximizeWindow();
  pub unsafe fn MinimizeWindow();
  pub unsafe fn RestoreWindow();
  pub unsafe fn SetWindowMonitor(monitor: i32);
  pub unsafe fn GetScreenWidth() -> i32;
  pub unsafe fn GetScreenHeight() -> i32;
  pub unsafe fn GetMonitorCount() -> i32;
  pub unsafe fn GetCurrentMonitor() -> i32;
  pub unsafe fn GetMonitorPosition(monitor: i32) -> Vector2;
  pub unsafe fn GetMonitorWidth(monitor: i32) -> i32;
  pub unsafe fn GetMonitorHeight(monitor: i32) -> i32;
  pub unsafe fn GetMonitorPhysicalWidth(monitor: i32) -> i32;
  pub unsafe fn GetMonitorPhysicalHeight(monitor: i32) -> i32;
  pub unsafe fn GetMonitorRefreshRate(monitor: i32) -> i32;
  pub unsafe fn GetWindowPosition() -> Vector2;
  pub unsafe fn GetMonitorName(monitor: i32) -> *const i8;

  // Cursor-related functions
  pub unsafe fn ShowCursor();
  pub unsafe fn HideCursor();
  pub unsafe fn IsCursorHidden() -> bool;
  pub unsafe fn EnableCursor();
  pub unsafe fn DisableCursor();

  // Drawing-related funtions
  pub unsafe fn ClearBackground(color: Color);
  pub unsafe fn BeginDrawing();
  pub unsafe fn EndDrawing();
  pub unsafe fn BeginMode2D(camera: Camera2D);
  pub unsafe fn EndMode2D();
  pub unsafe fn BeginMode3D(camera: Camera3D);
  pub unsafe fn EndMode3D();
  pub unsafe fn BeginTextureMode(target: RenderTexture2D);
  pub unsafe fn EndTextureMode();
  pub unsafe fn BeginVrStereoMode(config: VrStereoConfig);
  pub unsafe fn EndVrStereoMode();

  // VR stereo config functions for VR simulator
  pub unsafe fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;

  // management: Shader  functions
  // NOTE: functionality: Shader  is not available on OpenGL 1.1
  pub unsafe fn LoadShader(vs_file_name: *const i8, fs_file_name: *const i8) -> Shader;
  pub unsafe fn GetShaderLocation(shader: Shader, uniform_name: *const i8) -> i32;
  pub unsafe fn SetShaderValue(
    shader: Shader,
    loc_index: i32,
    value: *const c_void,
    uniform_type: i32,
  );

  // Screen-space-related functions
  pub unsafe fn GetScreenToWorldRay(position: Vector2, camera: Camera3D) -> Ray;
  pub unsafe fn GetWorldToScreen(position: Vector3, camera: Camera3D) -> Vector2;

  // Timing-related functions
  pub unsafe fn SetTargetFPS(fps: i32);
  pub unsafe fn GetFrameTime() -> f32;
  pub unsafe fn GetFPS() -> i32;

  // Random values generation functions
  pub unsafe fn GetRandomValue(min: i32, max: i32) -> i32;

  // Misc. functions
  pub unsafe fn SetConfigFlags(flags: u32);

  // File system functions
  pub unsafe fn IsFileDropped() -> bool;
  pub unsafe fn LoadDroppedFiles() -> FilePathList;
  pub unsafe fn UnloadDroppedFiles(files: FilePathList);

  // Input-related functions: Keyboard
  pub unsafe fn IsKeyPressed(key: i32) -> bool;
  pub unsafe fn IsKeyDown(key: i32) -> bool;
  pub unsafe fn SetExitKey(key: i32);

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

  // Camera System Functions
  pub unsafe fn UpdateCamera(camera: *mut Camera3D, mode: i32);
}
