use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    keyboard::is_key_pressed, mouse::is_mouse_button_pressed, set_target_fps, window_should_close,
  },
  enums::{KeyboardKey, MouseButton},
  shape::{draw_rectangle, draw_rectangle_lines},
  structs::Texture,
  text::draw_text,
  texture::{
    draw_texture, fade, gen_image_cellular, gen_image_checked, gen_image_gradient_linear,
    gen_image_gradient_radial, gen_image_gradient_square, gen_image_perlin_noise,
    gen_image_white_noise, load_texture_from_image, unload_image, unload_texture,
  },
};

const NUM_TEXTURES: usize = 9;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image generation",
  );

  let vertical_gradient =
    gen_image_gradient_linear(SCREEN_WIDTH, SCREEN_HEIGHT, 0, colors::RED, colors::BLUE);
  let horizontal_gradient =
    gen_image_gradient_linear(SCREEN_WIDTH, SCREEN_HEIGHT, 90, colors::RED, colors::BLUE);
  let diagonal_gradient =
    gen_image_gradient_linear(SCREEN_WIDTH, SCREEN_HEIGHT, 45, colors::RED, colors::BLUE);
  let radial_gradient = gen_image_gradient_radial(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    0.0,
    colors::WHITE,
    colors::BLACK,
  );
  let square_gradient = gen_image_gradient_square(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    0.0,
    colors::WHITE,
    colors::BLACK,
  );
  let checked = gen_image_checked(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    32,
    32,
    colors::RED,
    colors::BLUE,
  );
  let white_noise = gen_image_white_noise(SCREEN_WIDTH, SCREEN_HEIGHT, 0.5);
  let perlin_noise = gen_image_perlin_noise(SCREEN_WIDTH, SCREEN_HEIGHT, 50, 50, 4.0);
  let cellular = gen_image_cellular(SCREEN_WIDTH, SCREEN_HEIGHT, 32);

  let textures: [Texture; NUM_TEXTURES] = [
    load_texture_from_image(vertical_gradient),
    load_texture_from_image(horizontal_gradient),
    load_texture_from_image(diagonal_gradient),
    load_texture_from_image(radial_gradient),
    load_texture_from_image(square_gradient),
    load_texture_from_image(checked),
    load_texture_from_image(white_noise),
    load_texture_from_image(perlin_noise),
    load_texture_from_image(cellular),
  ];

  unload_image(vertical_gradient);
  unload_image(horizontal_gradient);
  unload_image(diagonal_gradient);
  unload_image(radial_gradient);
  unload_image(square_gradient);
  unload_image(checked);
  unload_image(white_noise);
  unload_image(perlin_noise);
  unload_image(cellular);

  let mut current_texture = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyboardKey::KeyRight) {
      current_texture = (current_texture + 1) % NUM_TEXTURES;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(textures[current_texture], 0, 0, colors::WHITE);

    draw_rectangle(30, 400, 325, 30, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(30, 400, 325, 30, fade(colors::WHITE, 0.5));
    draw_text(
      "MOUSE LEFT BUTTON to CYCLE PROCEDURAL TEXTURES",
      40,
      410,
      10,
      colors::WHITE,
    );

    match current_texture {
      0 => draw_text("VERTICAL GRADIENT", 560, 10, 20, colors::RAYWHITE),
      1 => draw_text("HORIZONTAL GRADIENT", 540, 10, 20, colors::RAYWHITE),
      2 => draw_text("DIAGONAL GRADIENT", 540, 10, 20, colors::RAYWHITE),
      3 => draw_text("RADIAL GRADIENT", 580, 10, 20, colors::LIGHTGRAY),
      4 => draw_text("SQUARE GRADIENT", 580, 10, 20, colors::LIGHTGRAY),
      5 => draw_text("CHECKED", 680, 10, 20, colors::RAYWHITE),
      6 => draw_text("WHITE NOISE", 640, 10, 20, colors::RED),
      7 => draw_text("PERLIN NOISE", 640, 10, 20, colors::RED),
      8 => draw_text("CELLULAR", 670, 10, 20, colors::RAYWHITE),
      _ => {}
    }

    end_drawing();
  }

  for i in 0..NUM_TEXTURES {
    unload_texture(textures[i]);
  }

  close_window();
}
