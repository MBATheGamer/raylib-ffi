fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  const MAX_COLORS: usize = 8;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - dashed line",
  );

  let line_start_position = Vector2 { x: 20.0, y: 50.0 };
  let line_end_position = Vector2 { x: 780.0, y: 400.0 };
  let dash_length = 25.0;
  let blank_length = 15.0;

  const LINE_COLORS: [Color; MAX_COLORS] = [
    colors::RED,
    colors::ORANGE,
    colors::GOLD,
    colors::GREEN,
    colors::BLUE,
    colors::VIOLET,
    colors::PINK,
    colors::BLACK,
  ];

  let color_index = 0;

  set_target_fps(60);

  while !window_should_close() {
    line_end_position = get_mouse_position();

    if is_key_down(KeyboardKey::KeyUp) {
      dash_length += 1.0;
    }
    if is_key_down(KeyboardKey::KeyDown) && dash_length > 1.0 {
      dash_length -= 1.0;
    }

    if is_key_down(KeyboardKey::KeyRight) {
      blank_length += 1.0;
    }
    if is_key_down(KeyboardKey::KeyLeft) && blank_length > 1.0 {
      blank_length -= 1.0;
    }

    if is_key_pressed(KeyboardKey::KeyC) {
      color_index = (color_index + 1) % LINE_COLORS.len();
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_line_dashed(
      line_start_position,
      line_end_position,
      dash_length as i32,
      blank_length as i32,
      LINE_COLORS[color_index],
    );

    draw_rectangle(5, 5, 265, 95, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(5, 5, 265, 95, colors::BLUE);

    draw_text("CONTROLS:", 15, 15, 10, colors::BLACK);
    draw_text("UP/DOWN: Change Dash Length", 15, 35, 10, colors::BLACK);
    draw_text("LEFT/RIGHT: Change Space Length", 15, 55, 10, colors::BLACK);
    draw_text("C: Cycle Color", 15, 75, 10, colors::BLACK);

    draw_text(
      &format!("Dash: {:.0} | Space: {:.0}", dash_length, blank_length),
      15,
      115,
      10,
      colors::DARKGRAY,
    );

    draw_fps(SCREEN_WIDTH - 80, 10);

    end_drawing();
  }

  close_window();
}
