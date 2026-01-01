fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image kernel",
  );

  let image = load_image("resources/cat.png");

  let gaussiankernel: [f32; _] = [1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];

  let sobelkernel: [f32; _] = [1.0, 0.0, -1.0, 2.0, 0.0, -2.0, 1.0, 0.0, -1.0];

  let sharpenkernel: [f32; _] = [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];

  normalize_kernel(&gaussiankernel);
  normalize_kernel(&sharpenkernel);
  normalize_kernel(&sobelkernel);

  let cat_sharpend = image_copy(image);
  image_kernel_convolution(&cat_sharpend, &sharpenkernel, 9);

  let cat_sobel = image_copy(image);
  image_kernel_convolution(&cat_sobel, &sobelkernel, 9);

  let cat_gaussian = image_copy(image);

  for _ in 0..6 {
    image_kernel_convolution(&cat_gaussian, &gaussiankernel, 9);
  }

  image_crop(&image, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&cat_gaussian, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&cat_sobel, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&cat_sharpend, Rectangle::new(0.0, 0.0, 200.0, 450.0));

  let texture = load_texture_from_image(image);
  let cat_sharpend_texture = load_texture_from_image(cat_sharpend);
  let cat_sobel_texture = load_texture_from_image(cat_sobel);
  let cat_gaussian_texture = load_texture_from_image(cat_gaussian);

  unload_image(image);
  unload_image(cat_gaussian);
  unload_image(cat_sobel);
  unload_image(cat_sharpend);

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(cat_sharpend_texture, 0, 0, colors::WHITE);
    draw_texture(cat_sobel_texture, 200, 0, colors::WHITE);
    draw_texture(cat_gaussian_texture, 400, 0, colors::WHITE);
    draw_texture(texture, 600, 0, colors::WHITE);

    end_drawing();
  }

  unload_texture(texture);
  unload_texture(cat_gaussian_texture);
  unload_texture(cat_sobel_texture);
  unload_texture(cat_sharpend_texture);

  close_window();
}

fn normalize_kernel(kernel: &[f32]) {
  let sum = 0.0;
  for i in 0..kernel.len() {
    sum += kernel[i];
  }

  if sum != 0.0 {
    for i in 0..kernel.len() {
      kernel[i] /= sum;
    }
  }
}
