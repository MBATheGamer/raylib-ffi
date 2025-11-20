fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - collision area",
  );

  let box_a = Rectangle {
    x: 10.0,
    y: get_screen_height() as f32 / 2.0 - 50.0,
    width: 200.0,
    height: 100.0,
  };
  let box_aspeed_x = 4;

  let box_b = Rectangle {
    x: get_screen_width() as f32 / 2.0 - 30.0,
    y: get_screen_height() as f32 / 2.0 - 30.0,
    width: 60.0,
    height: 60.0,
  };

  let box_collision = Rectangle::default();

  let screen_upper_limit = 40;

  let pause = false;
  let collision = false;

  set_target_fps(60);

  while !window_should_close() {
    if !pause {
      box_a.x += box_aspeed_x as f32;
    }

    if (box_a.x + box_a.width) >= get_screen_width() as f32 || box_a.x <= 0.0 {
      box_aspeed_x *= -1;
    }

    box_b.x = get_mouse_x() as f32 - box_b.width / 2.0;
    box_b.y = get_mouse_y() as f32 - box_b.height / 2.0;

    if (box_b.x + box_b.width) >= get_screen_width() as f32 {
      box_b.x = get_screen_width() as f32 - box_b.width;
    } else if box_b.x <= 0.0 {
      box_b.x = 0.0;
    }

    if (box_b.y + box_b.height) >= get_screen_height() as f32 {
      box_b.y = get_screen_height() as f32 - box_b.height;
    } else if box_b.y <= screen_upper_limit as f32 {
      box_b.y = screen_upper_limit as f32;
    }

    collision = check_collision_recs(box_a, box_b);

    if collision {
      box_collision = get_collision_rec(box_a, box_b);
    }

    if is_key_pressed(KeyboardKey::KeySpace) {
      pause = !pause;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_rectangle(
      0,
      0,
      SCREEN_WIDTH,
      screen_upper_limit,
      if collision {
        colors::RED
      } else {
        colors::BLACK
      },
    );

    draw_rectangle_rec(box_a, colors::GOLD);
    draw_rectangle_rec(box_b, colors::BLUE);

    if collision {
      draw_rectangle_rec(box_collision, colors::LIME);

      draw_text(
        "COLLISION!",
        get_screen_width() / 2 - measure_text("COLLISION!", 20) / 2,
        screen_upper_limit / 2 - 10,
        20,
        colors::BLACK,
      );

      draw_text(
        &format!(
          "Collision Area: {:.0}",
          box_collision.width * box_collision.height
        ),
        get_screen_width() / 2 - 100,
        screen_upper_limit + 10,
        20,
        colors::BLACK,
      );
    }

    draw_text(
      "Press SPACE to PAUSE/RESUME",
      20,
      SCREEN_HEIGHT - 35,
      20,
      colors::LIGHTGRAY,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
