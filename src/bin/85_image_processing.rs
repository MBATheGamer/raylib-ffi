const NUM_PROCESSES: usize = 9;

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

  let img_origin = load_image("resources/parrots.png");
  image_format(&img_origin, PixelFormat::UncompressedR8G8B8A8);
  let texture = load_texture_from_image(img_origin);

  let img_copy = image_copy(img_origin);

  let current_process = ImageProcess::None;
  let texture_reload = false;

  let toggle_recs: [Rectangle; NUM_PROCESSES] = [Rectangle::default(); NUM_PROCESSES];
  let mouse_hover_rec = -1;

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
          current_process = i;
          texture_reload = true;
        }
        break;
      } else {
        mouse_hover_rec = -1;
      }
    }

    if is_key_pressed(KeyboardKey::KeyDown) {
      current_process += 1;
      if current_process > (NUM_PROCESSES - 1) {
        current_process = 0;
      }
      texture_reload = true;
    } else if is_key_pressed(KEY_UP) {
      current_process -= 1;
      if current_process < 0 {
        current_process = 7;
      }
      texture_reload = true;
    }

    if texture_reload {
      unload_image(img_copy);
      img_copy = image_copy(img_origin);

      match current_process {
        ImageProcess::ColorGrayscale => image_color_grayscale(&img_copy),
        ImageProcess::ColorTint => image_color_tint(&img_copy, colors::GREEN),
        ImageProcess::ColorInvert => image_color_invert(&img_copy),
        ImageProcess::ColorContrast => image_color_contrast(&img_copy, -40),
        ImageProcess::ColorBrightness => image_color_brightness(&img_copy, -80),
        ImageProcess::GaussianBlur => image_blur_gaussian(&img_copy, 10),
        ImageProcess::FlipVertical => image_flip_vertical(&img_copy),
        ImageProcess::FlipHorizontal => image_flip_horizontal(&img_copy),
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
        if i == current_process || i == mouse_hover_rec {
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
        if i == current_process || i == mouse_hover_rec {
          colors::BLUE
        } else {
          colors::GRAY
        },
      );
      draw_text(
        PROCESS_TEXT[i],
        (toggle_recs[i].x + toggle_recs[i].width / 2 - measure_text(PROCESS_TEXT[i], 10) / 2)
          as i32,
        toggle_recs[i].y as i32 + 11,
        10,
        if i == current_process || i == mouse_hover_rec {
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
