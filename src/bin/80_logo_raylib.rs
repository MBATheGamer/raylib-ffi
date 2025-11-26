fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - logo raylib",
  );

  let texture = load_texture("resources/raylib_logo.png");

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

    draw_text("this IS a texture!", 360, 370, 10, colors::GRAY);

    end_drawing();
  }

  unload_texture(texture);

  close_window();
}
