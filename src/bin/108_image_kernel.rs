use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  structs::Rectangle,
  texture::{
    draw_texture, image_copy, image_crop, image_kernel_convolution, load_image,
    load_texture_from_image, unload_image, unload_texture,
  },
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image kernel",
  );

  let mut image = load_image("resources/cat.png");

  let mut gaussiankernel = [1.0, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];

  let mut sobelkernel = [1.0, 0.0, -1.0, 2.0, 0.0, -2.0, 1.0, 0.0, -1.0];

  let mut sharpenkernel = [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];

  normalize_kernel(&mut gaussiankernel);
  normalize_kernel(&mut sharpenkernel);
  normalize_kernel(&mut sobelkernel);

  let mut cat_sharpend = image_copy(image);
  image_kernel_convolution(&mut cat_sharpend, &sharpenkernel, 9);

  let mut cat_sobel = image_copy(image);
  image_kernel_convolution(&mut cat_sobel, &sobelkernel, 9);

  let mut cat_gaussian = image_copy(image);

  for _ in 0..6 {
    image_kernel_convolution(&mut cat_gaussian, &gaussiankernel, 9);
  }

  image_crop(&mut image, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&mut cat_gaussian, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&mut cat_sobel, Rectangle::new(0.0, 0.0, 200.0, 450.0));
  image_crop(&mut cat_sharpend, Rectangle::new(0.0, 0.0, 200.0, 450.0));

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

fn normalize_kernel(kernel: &mut [f32]) {
  let mut sum = 0.0;
  for i in 0..kernel.len() {
    sum += kernel[i];
  }

  if sum != 0.0 {
    for i in 0..kernel.len() {
      kernel[i] /= sum;
    }
  }
}
