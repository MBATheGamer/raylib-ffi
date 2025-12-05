const MAX_PARTICLES: usize = 200;

#[derive(Clone, Copy, Default)]
struct Particle {
  position: Vector2,
  color: Color,
  alpha: f32,
  size: f32,
  rotation: f32,
  active: bool,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - particles blending",
  );

  let mouse_tail: [Particle; MAX_PARTICLES] = [Particle::default(); MAX_PARTICLES];

  for i in 0..MAX_PARTICLES {
    mouse_tail[i].position = Vector2 { x: 0.0, y: 0.0 };
    mouse_tail[i].color = Color {
      red: get_random_value(0, 255) as u8,
      green: get_random_value(0, 255) as u8,
      blue: get_random_value(0, 255) as u8,
      alpha: 255,
    };
    mouse_tail[i].alpha = 1.0;
    mouse_tail[i].size = get_random_value(1, 30) as f32 / 20.0;
    mouse_tail[i].rotation = get_random_value(0, 360) as f32;
    mouse_tail[i].active = false;
  }

  let gravity = 3.0;

  let smoke = load_texture("resources/spark_flame.png");

  let blending = BlendMode::Alpha;

  set_target_fps(60);

  while !window_should_close() {
    for i in 0..MAX_PARTICLES {
      if !mouse_tail[i].active {
        mouse_tail[i].active = true;
        mouse_tail[i].alpha = 1.0;
        mouse_tail[i].position = get_mouse_position();
        i = MAX_PARTICLES;
      }
    }

    for i in 0..MAX_PARTICLES {
      if mouse_tail[i].active {
        mouse_tail[i].position.y += gravity / 2.0;
        mouse_tail[i].alpha -= 0.005;

        if mouse_tail[i].alpha <= 0.0 {
          mouse_tail[i].active = false;
        }

        mouse_tail[i].rotation += 2.0;
      }
    }

    if is_key_pressed(KeyboardKey::KeySpace) {
      if blending == BlendMode::Alpha {
        blending = BlendMode::Additive;
      } else {
        blending = BlendMode::Alpha;
      }
    }

    begin_drawing();

    clear_background(colors::DARKGRAY);

    begin_blend_mode(blending);

    for i in 0..MAX_PARTICLES {
      if mouse_tail[i].active {
        draw_texture_pro(
          smoke,
          Rectangle {
            x: 0.0,
            y: 0.0,
            width: smoke.width as f32,
            height: smoke.height as f32,
          },
          Rectangle {
            x: mouse_tail[i].position.x,
            y: mouse_tail[i].position.y,
            width: smoke.width as f32 * mouse_tail[i].size,
            height: smoke.height as f32 * mouse_tail[i].size,
          },
          Vector2 {
            x: smoke.width as f32 * mouse_tail[i].size / 2.0,
            y: smoke.height as f32 * mouse_tail[i].size / 2.0,
          },
          mouse_tail[i].rotation,
          fade(mouse_tail[i].color, mouse_tail[i].alpha),
        );
      }
    }

    end_blend_mode();

    draw_text(
      "PRESS SPACE to CHANGE BLENDING MODE",
      180,
      20,
      20,
      colors::BLACK,
    );

    if blending == BlendMode::Alpha {
      draw_text("ALPHA BLENDING", 290, SCREEN_HEIGHT - 40, 20, colors::BLACK);
    } else {
      draw_text(
        "ADDITIVE BLENDING",
        280,
        SCREEN_HEIGHT - 40,
        20,
        colors::RAYWHITE,
      );
    }

    end_drawing();
  }

  unload_texture(smoke);

  close_window();
}
