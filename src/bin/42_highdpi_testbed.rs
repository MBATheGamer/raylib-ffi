use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  shape::draw_line_ex,
  structs::Vector2,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - highdpi testbed",
  );

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_line_ex(
      Vector2 { x: 0.0, y: 0.0 },
      Vector2 {
        x: SCREEN_WIDTH as f32,
        y: SCREEN_HEIGHT as f32,
      },
      2.0,
      colors::RED,
    );
    draw_line_ex(
      Vector2 {
        x: 0.0,
        y: SCREEN_HEIGHT as f32,
      },
      Vector2 {
        x: SCREEN_WIDTH as f32,
        y: 0.0,
      },
      2.0,
      colors::RED,
    );
    draw_text(
      "example base code template",
      260,
      400,
      20,
      colors::LIGHTGRAY,
    );

    end_drawing();
  }

  close_window();
}
