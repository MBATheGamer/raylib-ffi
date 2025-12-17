use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_random_value, init_window,
    keyboard::is_key_pressed, load_random_sequence, set_target_fps, unload_random_sequence,
    window_should_close,
  },
  enums::KeyboardKey,
  math::remap,
  shape::draw_rectangle_rec,
  structs::{Color, Rectangle},
  text::{draw_fps, draw_text},
};

#[derive(Clone, Copy)]
struct ColorRect {
  color: Color,
  rect: Rectangle,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - random sequence",
  );

  let mut rect_count = 20;
  let mut rect_size = SCREEN_WIDTH as f32 / rect_count as f32;
  let mut rectangles = generate_random_color_rect_sequence(
    rect_count as f32,
    rect_size,
    SCREEN_WIDTH as f32,
    0.75 * SCREEN_HEIGHT as f32,
  );

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeySpace) {
      shuffle_color_rect_sequence(&mut rectangles);
    }

    if is_key_pressed(KeyboardKey::KeyUp) {
      rect_count += 1;
      rect_size = SCREEN_WIDTH as f32 / rect_count as f32;
      rectangles = generate_random_color_rect_sequence(
        rect_count as f32,
        rect_size,
        SCREEN_WIDTH as f32,
        0.75 * SCREEN_HEIGHT as f32,
      );
    }

    if is_key_pressed(KeyboardKey::KeyDown) {
      if rect_count >= 4 {
        rect_count -= 1;
        rect_size = SCREEN_WIDTH as f32 / rect_count as f32;
        rectangles = generate_random_color_rect_sequence(
          rect_count as f32,
          rect_size,
          SCREEN_WIDTH as f32,
          0.75 * SCREEN_HEIGHT as f32,
        );
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    for i in 0..rect_count {
      draw_rectangle_rec(rectangles[i].rect, rectangles[i].color);

      draw_text(
        "Press SPACE to shuffle the sequence",
        10,
        SCREEN_HEIGHT - 96,
        20,
        colors::BLACK,
      );

      draw_text(
        "Press UP to add a rectangle and generate a new sequence",
        10,
        SCREEN_HEIGHT - 64,
        20,
        colors::BLACK,
      );

      draw_text(
        "Press DOWN to remove a rectangle and generate a new sequence",
        10,
        SCREEN_HEIGHT - 32,
        20,
        colors::BLACK,
      );
    }

    draw_text(
      &format!("Count: {} rectangles", rect_count),
      10,
      10,
      20,
      colors::MAROON,
    );

    draw_fps(SCREEN_WIDTH - 80, 10);

    end_drawing();
  }

  close_window();
}

fn generate_random_color() -> Color {
  return Color {
    red: get_random_value(0, 255) as u8,
    green: get_random_value(0, 255) as u8,
    blue: get_random_value(0, 255) as u8,
    alpha: 255,
  };
}

fn generate_random_color_rect_sequence(
  rect_count: f32,
  rect_width: f32,
  screen_width: f32,
  screen_height: f32,
) -> Vec<ColorRect> {
  let mut rectangles: Vec<ColorRect> = vec![];

  let mut seq = load_random_sequence(rect_count as usize, 0, rect_count as i32 - 1);
  let rect_seq_width = rect_count * rect_width;
  let start_x = (screen_width - rect_seq_width) * 0.5;

  for i in 0..rect_count as usize {
    let rect_height = remap(seq[i] as f32, 0.0, rect_count - 1.0, 0.0, screen_height) as f32;

    rectangles.push(ColorRect {
      color: generate_random_color(),
      rect: Rectangle {
        x: start_x + i as f32 * rect_width,
        y: screen_height - rect_height,
        width: rect_width,
        height: rect_height,
      },
    });
  }

  unload_random_sequence(&mut seq);
  std::mem::forget(seq);

  return rectangles;
}

fn shuffle_color_rect_sequence(rectangles: &mut Vec<ColorRect>) {
  let mut seq = load_random_sequence(rectangles.len(), 0, rectangles.len() as i32 - 1);

  for i in 0..rectangles.len() {
    let tmp = rectangles[i];
    rectangles[i] = rectangles[seq[i] as usize];
    rectangles[seq[i] as usize] = tmp;
  }

  unload_random_sequence(&mut seq);
  std::mem::forget(seq);
}
