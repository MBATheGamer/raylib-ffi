use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  shape::draw_rectangle_lines,
  structs::{Rectangle, Vector2},
  text::{draw_text, load_font, unload_font},
  texture::{
    draw_texture, image_crop, image_draw, image_draw_circle_lines, image_draw_pixel,
    image_draw_rectangle, image_draw_text_ex, image_flip_horizontal, image_resize, load_image,
    load_texture_from_image, unload_image, unload_texture,
  },
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image drawing",
  );

  let mut cat = load_image("resources/cat.png");
  image_crop(
    &mut cat,
    Rectangle {
      x: 100.0,
      y: 10.0,
      width: 280.0,
      height: 380.0,
    },
  );
  image_flip_horizontal(&mut cat);
  image_resize(&mut cat, 150, 200);

  let mut parrots = load_image("resources/parrots.png");

  image_draw(
    &mut parrots,
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

  let crop = Rectangle {
    x: 0.0,
    y: 50.0,
    width: parrots.width as f32,
    height: parrots.height as f32 - 100.0,
  };

  image_crop(&mut parrots, crop);

  image_draw_pixel(&mut parrots, 10, 10, colors::RAYWHITE);
  image_draw_circle_lines(&mut parrots, 10, 10, 5, colors::RAYWHITE);
  image_draw_rectangle(&mut parrots, 5, 20, 10, 10, colors::RAYWHITE);

  unload_image(cat);

  let font = load_font("resources/custom_jupiter_crash.png");

  image_draw_text_ex(
    &mut parrots,
    font,
    "PARROTS & CAT",
    Vector2 { x: 300.0, y: 230.0 },
    font.base_size as f32,
    -2.0,
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
