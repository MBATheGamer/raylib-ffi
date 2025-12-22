use raylib_ffi::{
  consts::colors,
  core::{
    begin_blend_mode, begin_drawing, clear_background, close_window, end_blend_mode, end_drawing,
    init_window, keyboard::is_key_pressed, set_target_fps, window_should_close,
  },
  enums::{BlendMode, KeyboardKey},
  text::draw_text,
  texture::{draw_texture, load_image, load_texture_from_image, unload_image, unload_texture},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - blend modes",
  );

  let bg_image = load_image("resources/cyberpunk_street_background.png");
  let bg_texture = load_texture_from_image(bg_image);

  let fg_image = load_image("resources/cyberpunk_street_foreground.png");
  let fg_texture = load_texture_from_image(fg_image);

  unload_image(bg_image);
  unload_image(fg_image);

  let mut blend_mode = BlendMode::Alpha;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeySpace) {
      blend_mode = match blend_mode {
        BlendMode::Alpha => BlendMode::Additive,
        BlendMode::Additive => BlendMode::Multiplied,
        BlendMode::Multiplied => BlendMode::AddColors,
        _ => BlendMode::Alpha,
      };
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(
      bg_texture,
      SCREEN_WIDTH / 2 - bg_texture.width / 2,
      SCREEN_HEIGHT / 2 - bg_texture.height / 2,
      colors::WHITE,
    );

    begin_blend_mode(blend_mode);
    draw_texture(
      fg_texture,
      SCREEN_WIDTH / 2 - fg_texture.width / 2,
      SCREEN_HEIGHT / 2 - fg_texture.height / 2,
      colors::WHITE,
    );
    end_blend_mode();

    draw_text(
      "Press SPACE to change blend modes.",
      310,
      350,
      10,
      colors::GRAY,
    );

    match blend_mode {
      BlendMode::Alpha => draw_text(
        "Current: BLEND_ALPHA",
        (SCREEN_WIDTH / 2) - 60,
        370,
        10,
        colors::GRAY,
      ),
      BlendMode::Additive => draw_text(
        "Current: BLEND_ADDITIVE",
        (SCREEN_WIDTH / 2) - 60,
        370,
        10,
        colors::GRAY,
      ),
      BlendMode::Multiplied => draw_text(
        "Current: BLEND_MULTIPLIED",
        (SCREEN_WIDTH / 2) - 60,
        370,
        10,
        colors::GRAY,
      ),
      BlendMode::AddColors => draw_text(
        "Current: BLEND_ADD_COLORS",
        (SCREEN_WIDTH / 2) - 60,
        370,
        10,
        colors::GRAY,
      ),
      _ => {}
    }

    draw_text(
      "(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)",
      SCREEN_WIDTH - 330,
      SCREEN_HEIGHT - 20,
      10,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_texture(fg_texture);
  unload_texture(bg_texture);

  close_window();
}
