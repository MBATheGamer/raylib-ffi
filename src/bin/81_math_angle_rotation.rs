fn main() {
  const SCREEN_WIDTH: i32 = 720;
  const SCREEN_HEIGHT: i32 = 400;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - math angle rotation",
  );
  set_target_fps(60);

  let center = Vector2 {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
  };
  let line_length = 150.0;

  let angles: [f32; 4] = [0.0, 30.0, 60.0, 90.0];

  let total_angle: f32 = 0.0;

  while !window_should_close() {
    total_angle += 1.0;
    if total_angle >= 360.0 {
      total_angle -= 360.0;
    }

    begin_drawing();
    clear_background(colors::WHITE);

    draw_text(
      "Fixed angles + rotating line",
      10,
      10,
      20,
      colors::LIGHTGRAY,
    );

    for i in 0..angles.len() {
      let rad = angles[i].to_radians();
      let end = Vector2 {
        x: center.x + rad.cos() * line_length,
        y: center.y + rad.sin() * line_length,
      };

      let col = match i {
        0 => colors::GREEN,
        1 => colors::ORANGE,
        2 => colors::BLUE,
        3 => colors::MAGENTA,
        _ => colors::WHITE,
      };

      draw_line_ex(center, end, 5.0, col);

      let text_pos = Vector2 {
        x: center.x + rad.cos() * (line_length + 20.0),
        y: center.y + rad.sin() * (line_length + 20.0),
      };
      draw_text(
        &format!("{}°", angles[i]),
        text_pos.x as i32,
        text_pos.y as i32,
        20,
        col,
      );
    }

    let anim_rad = total_angle.to_radians();
    let anim_end = Vector2 {
      x: center.x + anim_rad.cos() * line_length,
      y: center.y + anim_rad.sin() * line_length,
    };

    let anim_col = color_from_hsv(total_angle % 360.0, 0.8, 0.9);
    draw_line_ex(center, anim_end, 5.0, anim_col);

    end_drawing();
  }

  close_window();
}
