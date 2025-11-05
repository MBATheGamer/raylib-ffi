use crate::structs::{Color, Font, Vector2};

unsafe extern "C" {
  // Font loading/unloading functions
  pub unsafe fn GetFontDefault() -> Font;

  // Text drawing functions
  pub unsafe fn DrawFPS(pos_x: i32, pos_y: i32);
  pub unsafe fn DrawText(text: *const i8, posX: i32, posY: i32, fontSize: i32, color: Color);
  pub unsafe fn DrawTextEx(
    font: Font,
    text: *const i8,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
  );

  // Text font info functions
  pub unsafe fn MeasureText(text: *const i8, font_size: i32) -> i32;
  pub unsafe fn MeasureTextEx(font: Font, text: *const i8, font_size: f32, spacing: f32)
  -> Vector2;
}
