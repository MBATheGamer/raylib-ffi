use crate::structs::Color;

unsafe extern "C" {
  // Text drawing functions
  pub unsafe fn DrawFPS(pos_x: i32, pos_y: i32);
  pub unsafe fn DrawText(text: *const i8, posX: i32, posY: i32, fontSize: i32, color: Color);

  // Text font info functions
  pub unsafe fn MeasureText(text: *const i8, font_size: i32) -> i32;
}
