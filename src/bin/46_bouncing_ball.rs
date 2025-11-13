fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::MSAA4xHint]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - bouncing ball",
  );

  let ball_position = Vector2 {
    x: get_screen_width() as f32 / 2.0,
    y: get_screen_height() as f32 / 2.0,
  };
  let ball_speed = Vector2 { x: 5.0, y: 4.0 };
  let ball_radius = 20;
  let gravity = 0.2;

  let use_gravity = true;
  let pause = false;
  let frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyG) {
      use_gravity = !use_gravity;
    }
    if is_key_pressed(KeyboardKey::KeySpace) {
      pause = !pause;
    }

    if !pause {
      ball_position.x += ball_speed.x;
      ball_position.y += ball_speed.y;

      if use_gravity {
        ball_speed.y += gravity;
      }

      if (ball_position.x >= (get_screen_width() - ball_radius) as f32)
        || ball_position.x <= ball_radius as f32
      {
        ball_speed.x *= -1.0;
      }
      if (ball_position.y >= (get_screen_height() - ball_radius) as f32)
        || ball_position.y <= ball_radius as f32
      {
        ball_speed.y *= -0.95;
      }
    } else {
      frames_counter += 1;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_circle_v(ball_position, ball_radius as f32, colors::MAROON);
    draw_text(
      "PRESS SPACE to PAUSE BALL MOVEMENT",
      10,
      get_screen_height() - 25,
      20,
      colors::LIGHTGRAY,
    );

    if use_gravity {
      draw_text(
        "GRAVITY: ON (Press G to disable)",
        10,
        get_screen_height() - 50,
        20,
        colors::DARKGREEN,
      );
    } else {
      draw_text(
        "GRAVITY: OFF (Press G to enable)",
        10,
        get_screen_height() - 50,
        20,
        colors::RED,
      );
    }

    if pause && ((frames_counter / 30) % 2) != 0 {
      draw_text("PAUSED", 350, 200, 30, colors::GRAY);
    }

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
