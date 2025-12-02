use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, keyboard::is_key_down,
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  structs::Vector2,
  text::{draw_text, draw_text_ex, load_font_ex, unload_font},
  texture::{
    draw_texture, draw_texture_v, image_draw_text_ex, load_image, load_texture_from_image,
    unload_image, unload_texture,
  },
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image text",
  );

  let mut parrots = load_image("resources/parrots.png");

  let font = load_font_ex("resources/KAISG.ttf", 64, None, 0);

  image_draw_text_ex(
    &mut parrots,
    font,
    "[Parrots font drawing]",
    Vector2 { x: 20.0, y: 20.0 },
    font.base_size as f32,
    0.0,
    colors::RED,
  );

  let texture = load_texture_from_image(parrots);
  unload_image(parrots);

  let position = Vector2 {
    x: (SCREEN_WIDTH / 2 - texture.width / 2) as f32,
    y: (SCREEN_HEIGHT / 2 - texture.height / 2 - 20) as f32,
  };

  let mut show_font;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeySpace) {
      show_font = true;
    } else {
      show_font = false;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if !show_font {
      draw_texture_v(texture, position, colors::WHITE);

      draw_text_ex(
        font,
        "[Parrots font drawing]",
        Vector2 {
          x: position.x + 20.0,
          y: position.y + 20.0 + 280.0,
        },
        font.base_size as f32,
        0.0,
        colors::WHITE,
      );
    } else {
      draw_texture(
        font.texture,
        SCREEN_WIDTH / 2 - font.texture.width / 2,
        50,
        colors::BLACK,
      );
    }

    draw_text(
      "PRESS SPACE to SHOW FONT ATLAS USED",
      290,
      420,
      10,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  unload_texture(texture);

  unload_font(font);

  close_window();
}
