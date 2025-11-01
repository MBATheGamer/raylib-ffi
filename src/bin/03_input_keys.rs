use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, keyboard::is_key_down,
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::draw_circle_v,
  structs::Vector2,
  text::draw_text,
};

fn main() {
  let screen_width = 800;
  let screen_height = 450;

  init_window(
    screen_width,
    screen_height,
    "raylib [core] example - input keys",
  );

  let mut ball_position = Vector2 {
    x: screen_width as f32 / 2.0,
    y: screen_height as f32 / 2.0,
  };

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeyRight) {
      ball_position.x += 2.0;
    }
    if is_key_down(KeyboardKey::KeyLeft) {
      ball_position.x -= 2.0;
    }
    if is_key_down(KeyboardKey::KeyUp) {
      ball_position.y -= 2.0;
    }
    if is_key_down(KeyboardKey::KeyDown) {
      ball_position.y += 2.0;
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);
    draw_text(
      "move the ball with arraw keys",
      10,
      10,
      20,
      colors::DARKGRAY,
    );
    draw_circle_v(ball_position, 50.0, colors::MAROON);
    end_drawing();
  }

  close_window();
}
