use raylib_ffi::{
  consts::colors,
  core::{
    begin_blend_mode, begin_drawing, clear_background, close_window, end_blend_mode, end_drawing,
    get_random_value, init_window, keyboard::is_key_pressed, mouse::get_mouse_position,
    set_target_fps, window_should_close,
  },
  enums::{BlendMode, KeyboardKey},
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
  texture::{draw_texture_pro, fade, load_texture, unload_texture},
};

const MAX_PARTICLES: usize = 200;

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

  let mut mouse_tail: Vec<Particle> = vec![];
  mouse_tail.reserve(MAX_PARTICLES);

  for _ in 0..mouse_tail.capacity() {
    mouse_tail.push(Particle {
      position: Vector2 { x: 0.0, y: 0.0 },
      color: Color {
        red: get_random_value(0, 255) as u8,
        green: get_random_value(0, 255) as u8,
        blue: get_random_value(0, 255) as u8,
        alpha: 255,
      },
      alpha: 1.0,
      size: get_random_value(1, 30) as f32 / 20.0,
      rotation: get_random_value(0, 360) as f32,
      active: false,
    });
  }

  let gravity = 3.0;

  let smoke = load_texture("resources/spark_flame.png");

  let mut blending = BlendMode::Alpha;

  set_target_fps(60);

  while !window_should_close() {
    for tail in &mut mouse_tail {
      if !tail.active {
        tail.active = true;
        tail.alpha = 1.0;
        tail.position = get_mouse_position();
        break;
      }
    }

    for tail in &mut mouse_tail {
      if tail.active {
        tail.position.y += gravity / 2.0;
        tail.alpha -= 0.005;

        if tail.alpha <= 0.0 {
          tail.active = false;
        }

        tail.rotation += 2.0;
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

    for tail in &mouse_tail {
      if tail.active {
        draw_texture_pro(
          smoke,
          Rectangle {
            x: 0.0,
            y: 0.0,
            width: smoke.width as f32,
            height: smoke.height as f32,
          },
          Rectangle {
            x: tail.position.x,
            y: tail.position.y,
            width: smoke.width as f32 * tail.size,
            height: smoke.height as f32 * tail.size,
          },
          Vector2 {
            x: smoke.width as f32 * tail.size / 2.0,
            y: smoke.height as f32 * tail.size / 2.0,
          },
          tail.rotation,
          fade(tail.color, tail.alpha),
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
