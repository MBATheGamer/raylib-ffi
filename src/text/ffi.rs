use crate::structs::Color;

unsafe extern "C" {
  // Text drawing functions
  pub unsafe fn DrawText(text: *const i8, posX: i32, posY: i32, fontSize: i32, color: Color);
}
