const MAX_BALLS: usize = 5000;

struct Ball {
  pos: Vector2,
  vel: Vector2,
  ppos: Vector2,
  radius: f32,
  friction: f32,
  elasticity: f32,
  color: Color,
  grabbed: bool,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - ball physics",
  );

  let balls = vec![Ball {
    pos: Vector2::new(
      get_screen_width() as f32 / 2.0,
      get_screen_height() as f32 / 2.0,
    ),
    vel: Vector2::new(200.0, 200.0),
    ppos: Vector2::zero(),
    radius: 40.0,
    friction: 0.99,
    elasticity: 0.9,
    color: colors::BLUE,
    grabbed: false,
  }];

  let grabbed_ball_index = -1;
  let press_offset = Vector2::zero();

  let gravity = 100.0;

  set_target_fps(60);

  while !window_should_close() {
    let delta = get_frame_time();
    let mouse_pos = get_mouse_position();

    if is_mouse_button_pressed(MouseButton::Left) {
      for i in (0..balls.len()).rev() {
        press_offset.x = mouse_pos.x - balls[i].pos.x;
        press_offset.y = mouse_pos.y - balls[i].pos.y;

        if press_offset.x.hypot(press_offset.y) <= balls[i].radius {
          balls[i].grabbed = true;
          grabbed_ball_index = i as i32;
          break;
        }
      }
    }

    if is_mouse_button_released(MouseButton::Left) {
      if grabbed_ball_index > -1 {
        balls[grabbed_ball_index as usize].grabbed = false;
        grabbed_ball_index = -1;
      }
    }

    if is_mouse_button_pressed(MouseButton::Right)
      || (is_key_down(KeyboardKey::KeyLeftControl) && is_mouse_button_down(MouseButton::Right))
    {
      if balls.len() < MAX_BALLS {
        balls.push(Ball {
          pos: mouse_pos,
          vel: Vector2::new(
            get_random_value(-300, 300) as f32,
            get_random_value(-300, 300) as f32,
          ),
          ppos: Vector2::zero(),
          radius: (20 + get_random_value(0, 30)) as f32,
          friction: 0.99,
          elasticity: 0.9,
          color: Color {
            red: get_random_value(0, 255) as u8,
            green: get_random_value(0, 255) as u8,
            blue: get_random_value(0, 255) as u8,
            alpha: 255,
          },
          grabbed: false,
        });
      }
    }

    if is_mouse_button_pressed(MouseButton::Middle) {
      for ball in &balls {
        if !ball.grabbed {
          ball.vel = Vector2::new(
            get_random_value(-2000, 2000) as f32,
            get_random_value(-2000, 2000) as f32,
          );
        }
      }
    }

    gravity += get_mouse_wheel_move() * 5.0;

    for ball in &balls {
      if !ball.grabbed {
        ball.pos.x += ball.vel.x * delta;
        ball.pos.y += ball.vel.y * delta;

        if (ball.pos.x + ball.radius) >= SCREEN_WIDTH as f32 {
          ball.pos.x = SCREEN_WIDTH as f32 - ball.radius;
          ball.vel.x = -ball.vel.x * ball.elasticity;
        } else if (ball.pos.x - ball.radius) <= 0.0 {
          ball.pos.x = ball.radius;
          ball.vel.x = -ball.vel.x * ball.elasticity;
        }

        if (ball.pos.y + ball.radius) >= SCREEN_HEIGHT as f32 {
          ball.pos.y = SCREEN_HEIGHT as f32 - ball.radius;
          ball.vel.y = -ball.vel.y * ball.elasticity;
        } else if (ball.pos.y - ball.radius) <= 0.0 {
          ball.pos.y = ball.radius;
          ball.vel.y = -ball.vel.y * ball.elasticity;
        }

        ball.vel.x = ball.vel.x * ball.friction;

        ball.vel.y = ball.vel.y * ball.friction + gravity;
      } else {
        ball.pos.x = mouse_pos.x - press_offset.x;
        ball.pos.y = mouse_pos.y - press_offset.y;

        ball.vel.x = (ball.pos.x - ball.ppos.x) / delta;
        ball.vel.y = (ball.pos.y - ball.ppos.y) / delta;
        ball.ppos = ball.pos;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    for ball in &balls {
      draw_circle_v(ball.pos, ball.radius, ball.color);
      draw_circle_lines_v(ball.pos, ball.radius, colors::BLACK);
    }

    draw_text(
      "grab a ball by pressing with the mouse and throw it by releasing",
      10,
      10,
      10,
      colors::DARKGRAY,
    );
    draw_text(
      "right click to create new balls (keep left control pressed to create a lot)",
      10,
      30,
      10,
      colors::DARKGRAY,
    );
    draw_text(
      "use mouse wheel to change gravity",
      10,
      50,
      10,
      colors::DARKGRAY,
    );
    draw_text("middle click to shake", 10, 70, 10, colors::DARKGRAY);
    draw_text(
      &format!("BALL COUNT: {}", balls.len()),
      10,
      get_screen_height() - 70,
      20,
      colors::BLACK,
    );
    draw_text(
      &format!("GRAVITY: {:.2}", gravity),
      10,
      get_screen_height() - 40,
      20,
      colors::BLACK,
    );

    end_drawing();
  }

  close_window();
}
