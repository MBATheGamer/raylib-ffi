fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - random values",
  );

  let rand_value = get_random_value(-8, 5);

  let frames_counter = 0u32;

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
