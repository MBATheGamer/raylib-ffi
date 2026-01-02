fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image channel",
  );

  let fudesumi_image = load_image("resources/fudesumi.png");

  let image_alpha = image_from_channel(fudesumi_image, 3);
  image_alpha_mask(&image_alpha, image_alpha);

  let image_red = image_from_channel(fudesumi_image, 0);
  image_alpha_mask(&image_red, image_alpha);

  let image_green = image_from_channel(fudesumi_image, 1);
  image_alpha_mask(&image_green, image_alpha);

  let image_blue = image_from_channel(fudesumi_image, 2);
  image_alpha_mask(&image_blue, image_alpha);

  let background_image = gen_image_checked(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    SCREEN_WIDTH / 20,
    SCREEN_HEIGHT / 20,
    colors::ORANGE,
    colors::YELLOW,
  );

  let fudesumi_texture = load_texture_from_image(fudesumi_image);
  let texture_alpha = load_texture_from_image(image_alpha);
  let texture_red = load_texture_from_image(image_red);
  let texture_green = load_texture_from_image(image_green);
  let texture_blue = load_texture_from_image(image_blue);
  let background_texture = load_texture_from_image(background_image);

  unload_image(fudesumi_image);
  unload_image(image_alpha);
  unload_image(image_red);
  unload_image(image_green);
  unload_image(image_blue);
  unload_image(background_image);

  let fudesumi_rec = Rectangle::new(
    0.0,
    0.0,
    fudesumi_image.width as f32,
    fudesumi_image.height as f32,
  );

  let fudesumi_pos = Rectangle::new(
    50.0,
    10.0,
    fudesumi_image.width as f32 * 0.8,
    fudesumi_image.height as f32 * 0.8,
  );
  let red_pos = Rectangle::new(
    410.0,
    10.0,
    fudesumi_pos.width / 2.0,
    fudesumi_pos.height / 2.0,
  );
  let green_pos = Rectangle::new(
    600.0,
    10.0,
    fudesumi_pos.width / 2.0,
    fudesumi_pos.height / 2.0,
  );
  let blue_pos = Rectangle::new(
    410.0,
    230.0,
    fudesumi_pos.width / 2.0,
    fudesumi_pos.height / 2.0,
  );
  let alpha_pos = Rectangle::new(
    600.0,
    230.0,
    fudesumi_pos.width / 2.0,
    fudesumi_pos.height / 2.0,
  );

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    draw_texture(background_texture, 0, 0, colors::WHITE);
    draw_texture_pro(
      fudesumi_texture,
      fudesumi_rec,
      fudesumi_pos,
      Vector2::new(0.0, 0.0),
      0.0,
      colors::WHITE,
    );

    draw_texture_pro(
      texture_red,
      fudesumi_rec,
      red_pos,
      Vector2::new(0.0, 0.0),
      0.0,
      colors::RED,
    );
    draw_texture_pro(
      texture_green,
      fudesumi_rec,
      green_pos,
      Vector2::new(0.0, 0.0),
      0.0,
      colors::GREEN,
    );
    draw_texture_pro(
      texture_blue,
      fudesumi_rec,
      blue_pos,
      Vector2::new(0.0, 0.0),
      0.0,
      colors::BLUE,
    );
    draw_texture_pro(
      texture_alpha,
      fudesumi_rec,
      alpha_pos,
      Vector2::new(0.0, 0.0),
      0.0,
      colors::WHITE,
    );

    end_drawing();
  }

  unload_texture(background_texture);
  unload_texture(fudesumi_texture);
  unload_texture(texture_red);
  unload_texture(texture_green);
  unload_texture(texture_blue);
  unload_texture(texture_alpha);

  close_window();
}
