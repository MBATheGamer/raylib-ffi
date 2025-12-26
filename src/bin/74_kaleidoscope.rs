use std::sync::{LazyLock, Mutex};

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    init_window,
    mouse::{get_mouse_position, is_mouse_button_down},
    set_target_fps, window_should_close,
  },
  enums::MouseButton,
  shape::draw_line_ex,
  structs::{Camera2D, Vector2},
  text::{draw_fps, draw_text},
};

const MAX_DRAW_LINES: usize = 8192;

struct Line {
  start: Vector2,
  end: Vector2,
}

static LINES: LazyLock<Mutex<Vec<Line>>> = LazyLock::new(|| {
  return Mutex::new(vec![]);
});

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - kaleidoscope",
  );

  let symmetry = 6;
  let angle = 360.0 / symmetry as f32;
  let thickness = 3.0;
  let mut mouse_pos = Vector2::zero();
  let scale_vector = Vector2::new(1.0, -1.0);
  let offset = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);

  let camera = Camera2D {
    target: Vector2::zero(),
    offset: offset,
    rotation: 0.0,
    zoom: 1.0,
  };

  set_target_fps(20);

  while !window_should_close() {
    let mut lines = LINES.lock().expect("Expecting LINES unlocked");
    let prev_mouse_pos = mouse_pos;
    mouse_pos = get_mouse_position();

    let mut line_start = mouse_pos - offset;
    let mut line_end = prev_mouse_pos - offset;

    if is_mouse_button_down(MouseButton::Left) {
      let mut s = 0;

      while s < symmetry && lines.len() < (MAX_DRAW_LINES - 1) {
        line_start = line_start.rotate(angle.to_radians());
        line_end = line_end.rotate(angle.to_radians());

        lines.push(Line {
          start: line_start,
          end: line_end,
        });

        lines.push(Line {
          start: line_start * scale_vector,
          end: line_end * scale_vector,
        });

        s += 1;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_2d(camera);
    for _ in 0..symmetry {
      let mut i = 0;
      while i < lines.len() {
        draw_line_ex(lines[i].start, lines[i].end, thickness, colors::BLACK);
        draw_line_ex(
          lines[i + 1].start,
          lines[i + 1].end,
          thickness,
          colors::BLACK,
        );

        i += 2;
      }
    }
    end_mode_2d();

    draw_text(
      &format!("LINES: {}/{}", lines.len(), MAX_DRAW_LINES),
      10,
      SCREEN_HEIGHT - 30,
      20,
      colors::MAROON,
    );
    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
