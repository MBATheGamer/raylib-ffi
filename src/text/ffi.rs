use crate::structs::{Color, Font, Vector2};

unsafe extern "C" {
  // Font loading/unloading functions
  pub fn GetFontDefault() -> Font;
  pub fn LoadFont(file_name: *const i8) -> Font;
  pub fn LoadFontEx(
    file_name: *const i8,
    font_size: i32,
    codepoints: *const i32,
    codepoint_count: i32,
  ) -> Font;
  pub fn UnloadFont(font: Font);

  // Text drawing functions
  pub fn DrawFPS(pos_x: i32, pos_y: i32);
  pub fn DrawText(text: *const i8, posX: i32, posY: i32, fontSize: i32, color: Color);
  pub fn DrawTextEx(
    font: Font,
    text: *const i8,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
  );

  // Text font info functions
  pub fn MeasureText(text: *const i8, font_size: i32) -> i32;
  pub fn MeasureTextEx(font: Font, text: *const i8, font_size: f32, spacing: f32) -> Vector2;
}
