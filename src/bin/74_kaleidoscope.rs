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
  let mouse_pos = Vector2::zero();
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
    let lines = LINES.lock().expect("Expecting LINES unlocked");
    let prev_mouse_pos = mouse_pos;
    mouse_pos = get_mouse_position();

    let line_start = mouse_pos - offset;
    let line_end = prev_mouse_pos - offset;

    if is_mouse_button_down(MouseButton::Left) {
      let s = 0;

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
      let i = 0;
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
