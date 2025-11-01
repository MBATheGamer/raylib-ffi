use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    mouse::get_mouse_wheel_move, set_target_fps, window_should_close,
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
    "raylib [core] example - input mouse wheel",
  );

  let mut box_position_y = SCREEN_HEIGHT / 2 - 40;
  let scroll_speed = 4;

  set_target_fps(60);

  while !window_should_close() {
    box_position_y -= (get_mouse_wheel_move() * scroll_speed as f32) as i32;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_rectangle(
      SCREEN_WIDTH / 2 - 40,
      box_position_y,
      80,
      80,
      colors::MAROON,
    );

    draw_text(
      "Use mouse wheel to move the cube up and down!",
      10,
      10,
      20,
      colors::GRAY,
    );
    draw_text(
      &format!("Box position Y: {}", box_position_y),
      10,
      40,
      20,
      colors::LIGHTGRAY,
    );

    end_drawing();
  }

  close_window();
}
