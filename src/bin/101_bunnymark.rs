use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_random_value,
    get_screen_height, get_screen_width, init_window,
    mouse::{get_mouse_position, is_mouse_button_down},
    set_target_fps, window_should_close,
  },
  enums::MouseButton,
  shape::draw_rectangle,
  structs::{Color, Vector2},
  text::{draw_fps, draw_text},
  texture::{draw_texture, load_texture, unload_texture},
};

const MAX_BUNNIES: usize = 50000;

const MAX_BATCH_ELEMENTS: i32 = 8192;

struct Bunny {
  position: Vector2,
  speed: Vector2,
  color: Color,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - bunnymark",
  );

  let tex_bunny = load_texture("resources/wabbit_alpha.png");

  let mut bunnies: Vec<Bunny> = vec![];

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_down(MouseButton::Left) {
      for _ in 0..100 {
        if bunnies.len() < MAX_BUNNIES {
          bunnies.push(Bunny {
            position: get_mouse_position(),
            speed: Vector2 {
              x: get_random_value(-250, 250) as f32 / 60.0,
              y: get_random_value(-250, 250) as f32 / 60.0,
            },
            color: Color {
              red: get_random_value(50, 240) as u8,
              green: get_random_value(80, 240) as u8,
              blue: get_random_value(100, 240) as u8,
              alpha: 255,
            },
          });
        }
      }
    }

    for bunny in &mut bunnies {
      bunny.position.x += bunny.speed.x;
      bunny.position.y += bunny.speed.y;

      if (bunny.position.x as i32 + tex_bunny.width / 2) > get_screen_width()
        || (bunny.position.x as i32 + tex_bunny.width / 2) < 0
      {
        bunny.speed.x *= -1.0;
      }
      if (bunny.position.y as i32 + tex_bunny.height / 2) > get_screen_height()
        || (bunny.position.y as i32 + tex_bunny.height / 2 - 40) < 0
      {
        bunny.speed.y *= -1.0;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    for bunny in &bunnies {
      draw_texture(
        tex_bunny,
        bunny.position.x as i32,
        bunny.position.y as i32,
        bunny.color,
      );
    }

    draw_rectangle(0, 0, SCREEN_WIDTH, 40, colors::BLACK);
    draw_text(
      &format!("bunnies: {}", bunnies.len()),
      120,
      10,
      20,
      colors::GREEN,
    );
    draw_text(
      &format!(
        "batched draw calls: {}",
        1 + bunnies.len() as i32 / MAX_BATCH_ELEMENTS
      ),
      320,
      10,
      20,
      colors::MAROON,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  unload_texture(tex_bunny);

  close_window();
}
