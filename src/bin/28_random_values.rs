use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_random_value, init_window,
    set_target_fps, window_should_close,
  },
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - random values",
  );

  let mut rand_value = get_random_value(-8, 5);

  let mut frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    frames_counter += 1;

    if ((frames_counter / 120) % 2) == 1 {
      rand_value = get_random_value(-8, 5);
      frames_counter = 0;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text(
      "Every 2 seconds a new random value is generated:",
      130,
      100,
      20,
      colors::MAROON,
    );

    draw_text(&format!("{}", rand_value), 360, 180, 80, colors::LIGHTGRAY);

    end_drawing();
  }

  close_window();
}
