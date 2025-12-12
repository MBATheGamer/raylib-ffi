use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  shape::draw_rectangle,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - logo raylib",
  );

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_rectangle(
      SCREEN_WIDTH / 2 - 128,
      SCREEN_HEIGHT / 2 - 128,
      256,
      256,
      colors::BLACK,
    );
    draw_rectangle(
      SCREEN_WIDTH / 2 - 112,
      SCREEN_HEIGHT / 2 - 112,
      224,
      224,
      colors::RAYWHITE,
    );
    draw_text(
      "raylib",
      SCREEN_WIDTH / 2 - 44,
      SCREEN_HEIGHT / 2 + 48,
      50,
      colors::BLACK,
    );

    draw_text("this is NOT a texture!", 350, 370, 10, colors::GRAY);

    end_drawing();
  }

  close_window();
}
