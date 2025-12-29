const MAX_TRAIL_LENGTH: usize = 30;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - mouse trail",
  );

  let trail_positions: Vec<Vector2> = vec![];

  for _ in 0..MAX_TRAIL_LENGTH {
    trail_positions.push(Vector2::zero());
  }

  set_target_fps(60);

  while !window_should_close() {
    let mouse_position = get_mouse_position();

    for i in (1..MAX_TRAIL_LENGTH).rev() {
      trail_positions[i] = trail_positions[i - 1];
    }

    trail_positions[0] = mouse_position;

    begin_drawing();

    clear_background(colors::BLACK);

    for i in 0..MAX_TRAIL_LENGTH {
      if trail_positions[i].x != 0.0 || trail_positions[i].y != 0.0 {
        let ratio = (MAX_TRAIL_LENGTH - i) as f32 / MAX_TRAIL_LENGTH as f32;

        let trail_color = fade(colors::SKYBLUE, ratio * 0.5 + 0.5);

        let trail_radius = 15.0 * ratio;

        draw_circle_v(trail_positions[i], trail_radius, trail_color);
      }
    }

    draw_circle_v(mouse_position, 15.0, colors::WHITE);

    draw_text(
      "Move the mouse to see the trail effect!",
      10,
      SCREEN_HEIGHT - 30,
      20,
      colors::LIGHTGRAY,
    );

    end_drawing();
  }

  close_window();
}
