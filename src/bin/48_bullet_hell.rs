use std::f32::consts::PI;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode, get_fps, init_window,
    keyboard::{is_key_down, is_key_pressed},
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{
    draw_circle, draw_circle_lines, draw_circle_lines_v, draw_circle_v, draw_rectangle,
    draw_rectangle_pro,
  },
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
  texture::{draw_texture, load_render_texture, unload_render_texture},
};

const MAX_BULLETS: i32 = 500000;

struct Bullet {
  position: Vector2,
  acceleration: Vector2,

  disabled: bool,
  color: Color,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - bullet hell",
  );

  let mut bullets: Vec<Bullet> = vec![];
  let mut bullet_count = 0;
  let mut bullet_disabled_count = 0;
  let bullet_radius = 10;
  let mut bullet_speed = 3.0;
  let mut bullet_rows = 6;
  let bullet_color: [Color; 2] = [colors::RED, colors::BLUE];

  let mut base_direction = 0.0;
  let mut angle_increment = 5;

  let mut spawn_cooldown = 2;
  let mut spawn_cooldown_timer = 2;

  let mut magic_circle_rotation = 0;

  let bullet_texture = load_render_texture(24, 24);

  begin_texture_mode(bullet_texture);
  draw_circle(12, 12, bullet_radius as f32, colors::WHITE);
  draw_circle_lines(12, 12, bullet_radius as f32, colors::BLACK);
  end_texture_mode();

  let mut draw_in_performance_mode = true;

  set_target_fps(60);

  while !window_should_close() {
    if bullet_count >= MAX_BULLETS {
      bullet_count = 0;
      bullet_disabled_count = 0;
    }

    spawn_cooldown_timer -= 1;
    if spawn_cooldown_timer < 0 {
      spawn_cooldown_timer = spawn_cooldown;

      let degrees_per_row = 360.0 / bullet_rows as f32;
      for row in 0..bullet_rows {
        if bullet_count < MAX_BULLETS {
          let bullet_direction = base_direction + (degrees_per_row * row as f32);
          bullets.push(Bullet {
            position: Vector2 {
              x: SCREEN_WIDTH as f32 / 2.0,
              y: SCREEN_HEIGHT as f32 / 2.0,
            },
            acceleration: Vector2 {
              x: bullet_speed * (bullet_direction * PI / 180.0).cos(),
              y: bullet_speed * (bullet_direction * PI / 180.0).sin(),
            },
            disabled: false,
            color: bullet_color[row % 2],
          });

          bullet_count += 1;
        }
      }

      base_direction += angle_increment as f32;
    }

    for i in 0..bullet_count as usize {
      if !bullets[i].disabled {
        bullets[i].position.x += bullets[i].acceleration.x;
        bullets[i].position.y += bullets[i].acceleration.y;

        if bullets[i].position.x < -bullet_radius as f32 * 2.0
          || bullets[i].position.x > (SCREEN_WIDTH + bullet_radius * 2) as f32
          || bullets[i].position.y < -bullet_radius as f32 * 2.0
          || bullets[i].position.y > (SCREEN_HEIGHT + bullet_radius * 2) as f32
        {
          bullets[i].disabled = true;
          bullet_disabled_count += 1;
        }
      }
    }

    if (is_key_pressed(KeyboardKey::KeyRight) || is_key_pressed(KeyboardKey::KeyD))
      && bullet_rows < 359
    {
      bullet_rows += 1;
    }
    if (is_key_pressed(KeyboardKey::KeyLeft) || is_key_pressed(KeyboardKey::KeyA))
      && bullet_rows > 1
    {
      bullet_rows -= 1;
    }
    if is_key_pressed(KeyboardKey::KeyUp) || is_key_pressed(KeyboardKey::KeyW) {
      bullet_speed += 0.25;
    }
    if (is_key_pressed(KeyboardKey::KeyDown) || is_key_pressed(KeyboardKey::KeyS))
      && bullet_speed > 0.50
    {
      bullet_speed -= 0.25;
    }
    if is_key_pressed(KeyboardKey::KeyZ) && spawn_cooldown > 1 {
      spawn_cooldown -= 1;
    }
    if is_key_pressed(KeyboardKey::KeyX) {
      spawn_cooldown += 1;
    }
    if is_key_pressed(KeyboardKey::KeyEnter) {
      draw_in_performance_mode = !draw_in_performance_mode;
    }

    if is_key_down(KeyboardKey::KeySpace) {
      angle_increment += 1;
      angle_increment %= 360;
    }

    if is_key_pressed(KeyboardKey::KeyC) {
      bullet_count = 0;
      bullet_disabled_count = 0;
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    magic_circle_rotation += 1;
    draw_rectangle_pro(
      Rectangle {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0,
        width: 120.0,
        height: 120.0,
      },
      Vector2 { x: 60.0, y: 60.0 },
      magic_circle_rotation as f32,
      colors::PURPLE,
    );
    draw_rectangle_pro(
      Rectangle {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0,
        width: 120.0,
        height: 120.0,
      },
      Vector2 { x: 60.0, y: 60.0 },
      magic_circle_rotation as f32 + 45.0,
      colors::PURPLE,
    );
    draw_circle_lines(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 70.0, colors::BLACK);
    draw_circle_lines(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 50.0, colors::BLACK);
    draw_circle_lines(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, 30.0, colors::BLACK);

    if draw_in_performance_mode {
      for i in 0..bullet_count as usize {
        if !bullets[i].disabled {
          draw_texture(
            bullet_texture.texture,
            (bullets[i].position.x - bullet_texture.texture.width as f32 * 0.5) as i32,
            (bullets[i].position.y - bullet_texture.texture.height as f32 * 0.5) as i32,
            bullets[i].color,
          );
        }
      }
    } else {
      for i in 0..bullet_count as usize {
        if !bullets[i].disabled {
          draw_circle_v(bullets[i].position, bullet_radius as f32, bullets[i].color);
          draw_circle_lines_v(bullets[i].position, bullet_radius as f32, colors::BLACK);
        }
      }
    }

    draw_rectangle(
      10,
      10,
      280,
      150,
      Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 200,
      },
    );
    draw_text("Controls:", 20, 20, 10, colors::LIGHTGRAY);
    draw_text(
      "- Right/Left or A/D: Change rows number",
      40,
      40,
      10,
      colors::LIGHTGRAY,
    );
    draw_text(
      "- Up/Down or W/S: Change bullet speed",
      40,
      60,
      10,
      colors::LIGHTGRAY,
    );
    draw_text(
      "- Z or X: Change spawn cooldown",
      40,
      80,
      10,
      colors::LIGHTGRAY,
    );
    draw_text(
      "- Space (Hold): Change the angle increment",
      40,
      100,
      10,
      colors::LIGHTGRAY,
    );
    draw_text(
      "- Enter: Switch draw method (Performance)",
      40,
      120,
      10,
      colors::LIGHTGRAY,
    );
    draw_text("- C: Clear bullets", 40, 140, 10, colors::LIGHTGRAY);

    draw_rectangle(
      610,
      10,
      170,
      30,
      Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 200,
      },
    );
    if draw_in_performance_mode {
      draw_text("Draw method: draw_texture(*)", 620, 20, 10, colors::GREEN);
    } else {
      draw_text("Draw method: DrawCircle(*)", 620, 20, 10, colors::RED);
    }

    draw_rectangle(
      135,
      410,
      530,
      30,
      Color {
        red: 0,
        green: 0,
        blue: 0,
        alpha: 200,
      },
    );
    draw_text(
      &format!(
        "[ FPS: {}, Bullets: {}, Rows: {}, Bullet speed: {:.2}, Angle increment per frame: {}, Cooldown: {:.0} ]",
        get_fps(),
        bullet_count - bullet_disabled_count,
        bullet_rows,
        bullet_speed,
        angle_increment,
        spawn_cooldown,
      ),
      155,
      420,
      10,
      colors::GREEN,
    );

    end_drawing();
  }

  unload_render_texture(bullet_texture);

  close_window();
}
