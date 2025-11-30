use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  text::draw_text,
  texture::{draw_texture, load_image, load_texture_from_image, unload_image, unload_texture},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image loading",
  );

  let image = load_image("resources/raylib_logo.png");
  let texture = load_texture_from_image(image);
  unload_image(image);

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(
      texture,
      SCREEN_WIDTH / 2 - texture.width / 2,
      SCREEN_HEIGHT / 2 - texture.height / 2,
      colors::WHITE,
    );

    draw_text(
      "this IS a texture loaded from an image!",
      300,
      370,
      10,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_texture(texture);

  close_window();
}
