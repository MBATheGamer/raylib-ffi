use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    keyboard::is_key_pressed,
    mouse::{get_mouse_position, is_mouse_button_released},
    set_target_fps, window_should_close,
  },
  enums::{KeyboardKey, MouseButton, PixelFormat},
  shape::{check_collision_point_rec, draw_rectangle_lines, draw_rectangle_rec},
  structs::Rectangle,
  text::{draw_text, measure_text},
  texture::{
    draw_texture, image_blur_gaussian, image_color_brightness, image_color_contrast,
    image_color_grayscale, image_color_invert, image_color_tint, image_copy, image_flip_horizontal,
    image_flip_vertical, image_format, load_image, load_image_colors, load_texture_from_image,
    unload_image, unload_image_colors, unload_texture, update_texture,
  },
};

const NUM_PROCESSES: usize = 9;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum ImageProcess {
  None = 0,
  ColorGrayscale,
  ColorTint,
  ColorInvert,
  ColorContrast,
  ColorBrightness,
  GaussianBlur,
  FlipVertical,
  FlipHorizontal,
}

impl ImageProcess {
  fn get(index: i32) -> ImageProcess {
    return match index {
      1 => Self::ColorGrayscale,
      2 => Self::ColorTint,
      3 => Self::ColorInvert,
      4 => Self::ColorContrast,
      5 => Self::ColorBrightness,
      6 => Self::GaussianBlur,
      7 => Self::FlipVertical,
      8 | -1 => Self::FlipHorizontal,
      _ => Self::None,
    };
  }
}

const PROCESS_TEXT: [&'static str; 9] = [
  "NO PROCESSING",
  "COLOR GRAYSCALE",
  "COLOR TINT",
  "COLOR INVERT",
  "COLOR CONTRAST",
  "COLOR BRIGHTNESS",
  "GAUSSIAN BLUR",
  "FLIP VERTICAL",
  "FLIP HORIZONTAL",
];

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - image processing",
  );

  let mut img_origin = load_image("resources/parrots.png");
  image_format(&mut img_origin, PixelFormat::UncompressedR8G8B8A8);
  let texture = load_texture_from_image(img_origin);

  let mut img_copy = image_copy(img_origin);

  let mut current_process = ImageProcess::None;
  let mut texture_reload = false;

  let mut toggle_recs: [Rectangle; NUM_PROCESSES] = [Rectangle::default(); NUM_PROCESSES];
  let mut mouse_hover_rec = -1;

  for i in 0..NUM_PROCESSES {
    toggle_recs[i] = Rectangle {
      x: 40.0,
      y: 50.0 + 32.0 * i as f32,
      width: 150.0,
      height: 30.0,
    };
  }

  set_target_fps(60);

  while !window_should_close() {
    for i in 0..NUM_PROCESSES {
      if check_collision_point_rec(get_mouse_position(), toggle_recs[i]) {
        mouse_hover_rec = i as i32;

        if is_mouse_button_released(MouseButton::Left) {
          current_process = ImageProcess::get(i as i32);
          texture_reload = true;
        }
        break;
      } else {
        mouse_hover_rec = -1;
      }
    }

    if is_key_pressed(KeyboardKey::KeyDown) {
      current_process = ImageProcess::get(current_process as i32 + 1);
      texture_reload = true;
    } else if is_key_pressed(KeyboardKey::KeyUp) {
      current_process = ImageProcess::get(current_process as i32 - 1);
      texture_reload = true;
    }

    if texture_reload {
      unload_image(img_copy);
      img_copy = image_copy(img_origin);

      match current_process {
        ImageProcess::ColorGrayscale => image_color_grayscale(&mut img_copy),
        ImageProcess::ColorTint => image_color_tint(&mut img_copy, colors::GREEN),
        ImageProcess::ColorInvert => image_color_invert(&mut img_copy),
        ImageProcess::ColorContrast => image_color_contrast(&mut img_copy, -40.0),
        ImageProcess::ColorBrightness => image_color_brightness(&mut img_copy, -80),
        ImageProcess::GaussianBlur => image_blur_gaussian(&mut img_copy, 10),
        ImageProcess::FlipVertical => image_flip_vertical(&mut img_copy),
        ImageProcess::FlipHorizontal => image_flip_horizontal(&mut img_copy),
        _ => {}
      }

      let pixels = load_image_colors(img_copy);
      update_texture(texture, pixels);
      unload_image_colors(pixels);

      texture_reload = false;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text("IMAGE PROCESSING:", 40, 30, 10, colors::DARKGRAY);

    for i in 0..NUM_PROCESSES {
      draw_rectangle_rec(
        toggle_recs[i],
        if i == current_process as usize || i == mouse_hover_rec as usize {
          colors::SKYBLUE
        } else {
          colors::LIGHTGRAY
        },
      );
      draw_rectangle_lines(
        toggle_recs[i].x as i32,
        toggle_recs[i].y as i32,
        toggle_recs[i].width as i32,
        toggle_recs[i].height as i32,
        if i == current_process as usize || i == mouse_hover_rec as usize {
          colors::BLUE
        } else {
          colors::GRAY
        },
      );
      draw_text(
        PROCESS_TEXT[i],
        (toggle_recs[i].x + toggle_recs[i].width / 2.0
          - measure_text(PROCESS_TEXT[i], 10) as f32 / 2.0) as i32,
        toggle_recs[i].y as i32 + 11,
        10,
        if i == current_process as usize || i == mouse_hover_rec as usize {
          colors::DARKBLUE
        } else {
          colors::DARKGRAY
        },
      );
    }

    draw_texture(
      texture,
      SCREEN_WIDTH - texture.width - 60,
      SCREEN_HEIGHT / 2 - texture.height / 2,
      colors::WHITE,
    );
    draw_rectangle_lines(
      SCREEN_WIDTH - texture.width - 60,
      SCREEN_HEIGHT / 2 - texture.height / 2,
      texture.width,
      texture.height,
      colors::BLACK,
    );

    end_drawing();
  }

  unload_texture(texture);
  unload_image(img_origin);
  unload_image(img_copy);

  close_window();
}
