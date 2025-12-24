const STAR_COUNT: usize = 420;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - starfield effect",
  );

  let bg_color = color_lerp(colors::DARKBLUE, colors::BLACK, 0.69);

  let speed = 10.0 / 9.0;

  let draw_lines = true;

  let stars: Vec<Vector3> = vec![];
  let stars_screen_pos: Vec<Vector2> = vec![];

  for _ in 0..STAR_COUNT {
    stars.push(Vector3 {
      x: get_random_value(-SCREEN_WIDTH / 2, SCREEN_WIDTH / 2) as f32,
      y: get_random_value(-SCREEN_HEIGHT / 2, SCREEN_HEIGHT / 2) as f32,
      z: 1.0,
    });

    stars_screen_pos.push(Vector2 { x: 0.0, y: 0.0 });
  }

  set_target_fps(60);

  while !window_should_close() {
    let mouse_move = get_mouse_wheel_move();
    if mouse_move as i32 != 0 {
      speed += 2.0 * mouse_move / 9.0;
    }
    if speed < 0.0 {
      speed = 0.1;
    } else if speed > 2.0 {
      speed = 2.0;
    }

    if is_key_pressed(KeyboardKey::KeySpace) {
      draw_lines = !draw_lines;
    }

    let dt = get_frame_time();
    for i in 0..STAR_COUNT {
      stars[i].z -= dt * speed;

      stars_screen_pos[i] = Vector2 {
        x: SCREEN_WIDTH as f32 * 0.5 + stars[i].x / stars[i].z,
        y: SCREEN_HEIGHT as f32 * 0.5 + stars[i].y / stars[i].z,
      };

      if stars[i].z < 0.0
        || stars_screen_pos[i].x < 0.0
        || stars_screen_pos[i].y < 0.0
        || stars_screen_pos[i].x > SCREEN_WIDTH as f32
        || stars_screen_pos[i].y > SCREEN_HEIGHT as f32
      {
        stars[i].x = get_random_value(-SCREEN_WIDTH / 2, SCREEN_WIDTH / 2) as f32;
        stars[i].y = get_random_value(-SCREEN_HEIGHT / 2, SCREEN_HEIGHT / 2) as f32;
        stars[i].z = 1.0;
      }
    }

    begin_drawing();

    clear_background(bg_color);

    for i in 0..STAR_COUNT {
      if draw_lines {
        let t = (stars[i].z + 1.0 / 32.0).clamp(0.0, 1.0);

        if (t - stars[i].z) > 1e-3 {
          let start_pos = Vector2 {
            x: SCREEN_WIDTH as f32 * 0.5 + stars[i].x / t,
            y: SCREEN_HEIGHT as f32 * 0.5 + stars[i].y / t,
          };

          draw_line_v(start_pos, stars_screen_pos[i], colors::RAYWHITE);
        }
      } else {
        let radius = lerp(stars[i].z, 1.0, 5.0);

        draw_circle_v(stars_screen_pos[i], radius, colors::RAYWHITE);
      }
    }

    draw_text(
      &format!("[MOUSE WHEEL] Current Speed: {:.0}", 9.0 * speed / 2.0),
      10,
      40,
      20,
      colors::RAYWHITE,
    );
    draw_text(
      &format!(
        "[SPACE] Current draw mode: {}",
        if draw_lines { "Lines" } else { "Circles" }
      ),
      10,
      70,
      20,
      colors::RAYWHITE,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
