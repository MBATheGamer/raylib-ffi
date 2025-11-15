use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_screen_height,
    get_screen_width, init_window, keyboard::is_key_down, mouse::get_mouse_position,
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{check_collision_point_rec, draw_rectangle, draw_rectangle_lines_ex, draw_rectangle_rec},
  structs::{Color, Rectangle},
  text::{draw_text, measure_text},
  texture::fade,
};

const MAX_COLORS_COUNT: usize = 21;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - colors palette",
  );

  const COLORS: [Color; MAX_COLORS_COUNT] = [
    colors::DARKGRAY,
    colors::MAROON,
    colors::ORANGE,
    colors::DARKGREEN,
    colors::DARKBLUE,
    colors::DARKPURPLE,
    colors::DARKBROWN,
    colors::GRAY,
    colors::RED,
    colors::GOLD,
    colors::LIME,
    colors::BLUE,
    colors::VIOLET,
    colors::BROWN,
    colors::LIGHTGRAY,
    colors::PINK,
    colors::YELLOW,
    colors::GREEN,
    colors::SKYBLUE,
    colors::PURPLE,
    colors::BEIGE,
  ];

  const COLOR_NAMES: [&'static str; MAX_COLORS_COUNT] = [
    "DARKGRAY",
    "MAROON",
    "ORANGE",
    "DARKGREEN",
    "DARKBLUE",
    "DARKPURPLE",
    "DARKBROWN",
    "GRAY",
    "RED",
    "GOLD",
    "LIME",
    "BLUE",
    "VIOLET",
    "BROWN",
    "LIGHTGRAY",
    "PINK",
    "YELLOW",
    "GREEN",
    "SKYBLUE",
    "PURPLE",
    "BEIGE",
  ];

  let mut colors_recs = [Rectangle::default(); MAX_COLORS_COUNT];

  for i in 0..MAX_COLORS_COUNT {
    colors_recs[i].x = 20.0 + 100.0 * (i % 7) as f32 + 10.0 * (i % 7) as f32;
    colors_recs[i].y = 80.0 + 100.0 * (i / 7) as f32 + 10.0 * (i / 7) as f32;
    colors_recs[i].width = 100.0;
    colors_recs[i].height = 100.0;
  }

  let mut color_state = [false; MAX_COLORS_COUNT];

  set_target_fps(60);

  while !window_should_close() {
    let mouse_point = get_mouse_position();

    for i in 0..MAX_COLORS_COUNT {
      if check_collision_point_rec(mouse_point, colors_recs[i]) {
        color_state[i] = true;
      } else {
        color_state[i] = false;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text("raylib colors palette", 28, 42, 20, colors::BLACK);
    draw_text(
      "press SPACE to see all colors",
      get_screen_width() - 180,
      get_screen_height() - 40,
      10,
      colors::GRAY,
    );

    for i in 0..MAX_COLORS_COUNT {
      draw_rectangle_rec(
        colors_recs[i],
        fade(COLORS[i], if color_state[i] { 0.6 } else { 1.0 }),
      );

      if is_key_down(KeyboardKey::KeySpace) || color_state[i] {
        draw_rectangle(
          colors_recs[i].x as i32,
          (colors_recs[i].y + colors_recs[i].height - 26.0) as i32,
          colors_recs[i].width as i32,
          20,
          colors::BLACK,
        );
        draw_rectangle_lines_ex(colors_recs[i], 6.0, fade(colors::BLACK, 0.3));
        draw_text(
          COLOR_NAMES[i],
          (colors_recs[i].x + colors_recs[i].width - measure_text(COLOR_NAMES[i], 10) as f32 - 12.0)
            as i32,
          (colors_recs[i].y + colors_recs[i].height - 20.0) as i32,
          10,
          COLORS[i],
        );
      }
    }

    end_drawing();
  }

  close_window();
}
