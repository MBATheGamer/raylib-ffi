fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image drawing",
  );

  let cat = load_image("resources/cat.png");
  image_crop(
    &cat,
    Rectangle {
      x: 100.0,
      y: 10.0,
      width: 280.0,
      height: 380.0,
    },
  );
  image_flip_horizontal(&cat);
  image_resize(&cat, 150, 200);

  let parrots = load_image("resources/parrots.png");

  image_draw(
    &parrots,
    cat,
    Rectangle {
      x: 0.0,
      y: 0.0,
      width: cat.width as f32,
      height: cat.height as f32,
    },
    Rectangle {
      x: 30.0,
      y: 40.0,
      width: cat.width as f32 * 1.5,
      height: cat.height as f32 * 1.5,
    },
    colors::WHITE,
  );
  image_crop(
    &parrots,
    Rectangle {
      x: 0.0,
      y: 50.0,
      width: parrots.width as f32,
      height: parrots.height as f32 - 100,
    },
  );

  image_draw_pixel(&parrots, 10, 10, colors::RAYWHITE);
  image_draw_circleLines(&parrots, 10, 10, 5, colors::RAYWHITE);
  image_draw_rectangle(&parrots, 5, 20, 10, 10, colors::RAYWHITE);

  unload_image(cat);

  let font = load_font("resources/custom_jupiter_crash.png");

  image_drawTextEx(
    &parrots,
    font,
    "PARROTS & CAT",
    Vector2 { x: 300.0, y: 230.0 },
    font.baseSize as f32,
    -2,
    colors::WHITE,
  );

  unload_font(font);

  let texture = load_texture_from_image(parrots);
  unload_image(parrots);

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(
      texture,
      SCREEN_WIDTH / 2 - texture.width / 2,
      SCREEN_HEIGHT / 2 - texture.height / 2 - 40,
      colors::WHITE,
    );
    draw_rectangle_lines(
      SCREEN_WIDTH / 2 - texture.width / 2,
      SCREEN_HEIGHT / 2 - texture.height / 2 - 40,
      texture.width,
      texture.height,
      colors::DARKGRAY,
    );

    draw_text(
      "We are drawing only one texture from various images composed!",
      240,
      350,
      10,
      colors::DARKGRAY,
    );
    draw_text(
      "Source images have been cropped, scaled, flipped and copied one over the other.",
      190,
      370,
      10,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  unload_texture(texture);

  close_window();
}
