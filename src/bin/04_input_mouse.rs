fn main() {
  let screen_width = 800;
  let screen_height = 450;

  init_window(
    screen_width,
    screen_height,
    "raylib [core] example - input mouse",
  );

  let mut ball_position = Vector2 {
    x: -100.0,
    y: -100.0,
  };
  let ball_color: Color = colors::DARKBLUE;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyH) {
      if is_cursor_hidden() {
        show_cursor();
      } else {
        hide_cursor();
      }
    }

    ball_position = get_mouse_position();

    if is_mouse_button_pressed(MouseButton::Left) {
      ball_color = colors::MAROON;
    } else if is_mouse_button_pressed(MouseButton::Middle) {
      ball_color = colors::LIME;
    } else if is_mouse_button_pressed(MouseButton::Right) {
      ball_color = colors::DARKBLUE;
    } else if is_mouse_button_pressed(MouseButton::Side) {
      ball_color = colors::PURPLE;
    } else if is_mouse_button_pressed(MouseButton::Extra) {
      ball_color = colors::YELLOW;
    } else if is_mouse_button_pressed(MouseButton::Forward) {
      ball_color = colors::ORANGE;
    } else if is_mouse_button_pressed(MouseButton::Back) {
      ball_color = colors::BEIGE;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_circle_v(ball_position, 40.0, ball_color);

    draw_text(
      "move ball with mouse and click mouse button to change color",
      10,
      10,
      20,
      colors::DARKGRAY,
    );
    draw_text(
      "Press 'H' to toggle cursor visibility",
      10,
      30,
      20,
      colors::DARKGRAY,
    );

    if is_cursor_hidden() {
      draw_text("CURSOR HIDDEN", 20, 60, 20, colors::RED);
    } else {
      draw_text("CURSOR VISIBLE", 20, 60, 20, colors::LIME);
    }
    end_drawing();
  }

  close_window();
}
