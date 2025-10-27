use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    keyboard::{is_key_pressed, set_exit_key},
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shapes::draw_rectangle,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - window should close",
  );

  set_exit_key(KeyboardKey::KeyNull);

  let mut exit_window_requested = false;
  let mut exit_window = false;

  set_target_fps(60);

  while !exit_window {
    if window_should_close() || is_key_pressed(KeyboardKey::KeyEscape) {
      exit_window_requested = true;
    }

    if exit_window_requested {
      if is_key_pressed(KeyboardKey::KeyY) {
        exit_window = true;
      } else if is_key_pressed(KeyboardKey::KeyN) {
        exit_window_requested = false;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if exit_window_requested {
      draw_rectangle(0, 100, SCREEN_WIDTH, 200, colors::BLACK);
      draw_text(
        "Are you sure you want to exit program? [Y/N]",
        40,
        180,
        30,
        colors::WHITE,
      );
    } else {
      draw_text(
        "Try to close the window to get confirmation message!",
        120,
        200,
        20,
        colors::LIGHTGRAY,
      );
    }

    end_drawing();
  }

  close_window();
}
