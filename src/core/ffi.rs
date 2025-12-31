use std::ffi::c_void;

use crate::structs::{
  AutomationEvent, AutomationEventList, Camera2D, Camera3D, Color, FilePathList, Image, Ray,
  RenderTexture, Shader, Vector2, Vector3, VrDeviceInfo, VrStereoConfig,
};

unsafe extern "C" {
  // Window-related functions
  pub fn InitWindow(width: i32, height: i32, title: *const i8);
  pub fn CloseWindow();
  pub fn WindowShouldClose() -> bool;
  pub fn IsWindowResized() -> bool;
  pub fn IsWindowState(flag: u32) -> bool;
  pub fn SetWindowState(flags: u32);
  pub fn ClearWindowState(flags: u32);
  pub fn ToggleFullscreen();
  pub fn ToggleBorderlessWindowed();
  pub fn MaximizeWindow();
  pub fn MinimizeWindow();
  pub fn RestoreWindow();
  pub fn SetWindowMonitor(monitor: i32);
  pub fn SetWindowMinSize(width: i32, height: i32);
  pub fn GetScreenWidth() -> i32;
  pub fn GetScreenHeight() -> i32;
  pub fn GetRenderWidth() -> i32;
  pub fn GetMonitorCount() -> i32;
  pub fn GetCurrentMonitor() -> i32;
  pub fn GetMonitorPosition(monitor: i32) -> Vector2;
  pub fn GetMonitorWidth(monitor: i32) -> i32;
  pub fn GetMonitorHeight(monitor: i32) -> i32;
  pub fn GetMonitorPhysicalWidth(monitor: i32) -> i32;
  pub fn GetMonitorPhysicalHeight(monitor: i32) -> i32;
  pub fn GetMonitorRefreshRate(monitor: i32) -> i32;
  pub fn GetWindowPosition() -> Vector2;
  pub fn GetWindowScaleDPI() -> Vector2;
  pub fn GetMonitorName(monitor: i32) -> *const i8;
  pub fn SetClipboardText(text: *const i8);
  pub fn GetClipboardText() -> *const i8;
  pub fn GetClipboardImage() -> Image;

  // Cursor-related functions
  pub fn ShowCursor();
  pub fn HideCursor();
  pub fn IsCursorHidden() -> bool;
  pub fn EnableCursor();
  pub fn DisableCursor();

  // Drawing-related funtions
  pub fn ClearBackground(color: Color);
  pub fn BeginDrawing();
  pub fn EndDrawing();
  pub fn BeginMode2D(camera: Camera2D);
  pub fn EndMode2D();
  pub fn BeginMode3D(camera: Camera3D);
  pub fn EndMode3D();
  pub fn BeginTextureMode(target: RenderTexture);
  pub fn EndTextureMode();
  pub fn BeginShaderMode(shader: Shader);
  pub fn EndShaderMode();
  pub fn BeginBlendMode(mode: i32);
  pub fn EndBlendMode();
  pub fn BeginScissorMode(x: i32, y: i32, width: i32, height: i32);
  pub fn EndScissorMode();
  pub fn BeginVrStereoMode(config: VrStereoConfig);
  pub fn EndVrStereoMode();

  // VR stereo config functions for VR simulator
  pub fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
  pub fn UnloadVrStereoConfig(config: VrStereoConfig);

  // management: Shader  functions
  // NOTE: functionality: Shader  is not available on OpenGL 1.1
  pub fn LoadShader(vs_file_name: *const i8, fs_file_name: *const i8) -> Shader;
  pub fn GetShaderLocation(shader: Shader, uniform_name: *const i8) -> i32;
  pub fn SetShaderValue(shader: Shader, loc_index: i32, value: *const c_void, uniform_type: i32);
  pub fn UnloadShader(shader: Shader);

  // Screen-space-related functions
  pub fn GetScreenToWorldRay(position: Vector2, camera: Camera3D) -> Ray;
  pub fn GetWorldToScreen(position: Vector3, camera: Camera3D) -> Vector2;
  pub fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
  pub fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;

  // Timing-related functions
  pub fn SetTargetFPS(fps: i32);
  pub fn GetFrameTime() -> f32;
  pub fn GetTime() -> f64;
  pub fn GetFPS() -> i32;

  // Custom frame control functions
  // NOTE: Those functions are intended for advanced users that want full control over the frame processing
  // By default EndDrawing() does this job: draws everything + SwapScreenBuffer() + manage frame timing + PollInputEvents()
  // To apub fn that behaviour and control frame processes manually, enable in config.h: SUPPORT_CUSTOM_FRAME_CONTROL
  pub fn SwapScreenBuffer();
  pub fn PollInputEvents();
  pub fn WaitTime(seconds: f64);

  // Random values generation functions
  pub fn GetRandomValue(min: i32, max: i32) -> i32;
  pub fn LoadRandomSequence(count: u32, min: i32, max: i32) -> *mut i32;
  pub fn UnloadRandomSequence(sequence: *mut i32);

  // Misc. functions
  pub fn SetConfigFlags(flags: u32);

  // NOTE: Following functions implemented in module [utils]
  pub fn SetTraceLogLevel(log_level: i32);

  // File system functions
  pub fn IsFileExtension(file_name: *const i8, ext: *const i8) -> bool;
  pub fn IsFileDropped() -> bool;
  pub fn LoadDroppedFiles() -> FilePathList;
  pub fn UnloadDroppedFiles(files: FilePathList);

  // Automation events functionality
  pub fn LoadAutomationEventList(file_name: *const i8) -> AutomationEventList;
  pub fn UnloadAutomationEventList(list: AutomationEventList);
  pub fn ExportAutomationEventList(list: AutomationEventList, file_name: *const i8) -> bool;
  pub fn PlayAutomationEvent(event: AutomationEvent);

  // Input-related functions: Keyboard
  pub fn IsKeyPressed(key: i32) -> bool;
  pub fn IsKeyDown(key: i32) -> bool;
  pub fn IsKeyReleased(key: i32) -> bool;
  pub fn SetExitKey(key: i32);

  // Input-related functions: Gamepads
  pub fn IsGamepadAvailable(gamepad: i32) -> bool;
  pub fn GetGamepadName(gamepad: i32) -> *const i8;
  pub fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool;
  pub fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool;
  pub fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool;
  pub fn GetGamepadButtonPressed() -> i32;
  pub fn GetGamepadAxisCount(gamepad: i32) -> i32;
  pub fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32;

  // Input-related functions: Mouse
  pub fn IsMouseButtonPressed(button: i32) -> bool;
  pub fn IsMouseButtonDown(button: i32) -> bool;
  pub fn IsMouseButtonReleased(button: i32) -> bool;
  pub fn GetMouseX() -> i32;
  pub fn GetMouseY() -> i32;
  pub fn GetMousePosition() -> Vector2;
  pub fn GetMouseDelta() -> Vector2;
  pub fn GetMouseWheelMove() -> f32;

  // Input-related functions: Touch
  pub fn GetTouchPosition(index: i32) -> Vector2;
  pub fn GetTouchPointCount() -> i32;

  // Gestures and touch handling functions
  pub fn GetGestureDetected() -> i32;
  pub fn IsGestureDetected(gesture: u32) -> bool;
  pub fn GetGestureDragAngle() -> f32;
  pub fn GetGesturePinchAngle() -> f32;

  // Camera System Functions
  pub fn UpdateCamera(camera: *mut Camera3D, mode: i32);
}
