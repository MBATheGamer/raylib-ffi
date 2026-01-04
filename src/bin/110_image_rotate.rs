fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image rotate",
  );

  let image45 = load_image("resources/raylib_logo.png");
  let image90 = load_image("resources/raylib_logo.png");
  let image_neg90 = load_image("resources/raylib_logo.png");

  image_rotate(&image45, 45);
  image_rotate(&image90, 90);
  image_rotate(&image_neg90, -90);

  let textures: Vec<Texture> = vec![
    load_texture_from_image(image45),
    load_texture_from_image(image90),
    load_texture_from_image(image_neg90),
  ];

  let current_texture = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyboardKey::KeyRight) {
      current_texture = (current_texture + 1) % textures.len();
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(
      textures[current_texture],
      SCREEN_WIDTH / 2 - textures[current_texture].width / 2,
      SCREEN_HEIGHT / 2 - textures[current_texture].height / 2,
      colors::WHITE,
    );

    draw_text(
      "Press LEFT MOUSE BUTTON to rotate the image clockwise",
      250,
      420,
      10,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  for i in 0..textures.len() {
    unload_texture(textures[i]);
  }

  close_window();
}
