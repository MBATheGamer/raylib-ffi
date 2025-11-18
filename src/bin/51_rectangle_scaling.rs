const MOUSE_SCALE_MARK_SIZE: i32 = 12;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - rectangle scaling",
  );

  let rec = Rectangle {
    x: 100.0,
    y: 100.0,
    width: 200.0,
    height: 80.0,
  };

  let mouse_scale_ready;
  let mouse_scale_mode = false;

  set_target_fps(60);

  while !window_should_close() {
    let mouse_position = get_mouse_position();

    if check_collision_point_rec(
      mouse_position,
      Rectangle {
        x: rec.x + rec.width - MOUSE_SCALE_MARK_SIZE as f32,
        y: rec.y + rec.height - MOUSE_SCALE_MARK_SIZE as f32,
        width: MOUSE_SCALE_MARK_SIZE as f32,
        height: MOUSE_SCALE_MARK_SIZE as f32,
      },
    ) {
      mouse_scale_ready = true;
      if is_mouse_button_pressed(MouseButton::Left) {
        mouse_scale_mode = true;
      }
    } else {
      mouse_scale_ready = false;
    }

    if mouse_scale_mode {
      mouse_scale_ready = true;

      rec.width = mouse_position.x - rec.x;
      rec.height = mouse_position.y - rec.y;

      if rec.width < MOUSE_SCALE_MARK_SIZE as f32 {
        rec.width = MOUSE_SCALE_MARK_SIZE as f32;
      }
      if rec.height < MOUSE_SCALE_MARK_SIZE as f32 {
        rec.height = MOUSE_SCALE_MARK_SIZE as f32;
      }

      // Check maximum rec size
      if rec.width > (get_screen_width() as f32 - rec.x) {
        rec.width = get_screen_width() as f32 - rec.x;
      }
      if rec.height > (get_screen_height() as f32 - rec.y) {
        rec.height = get_screen_height() as f32 - rec.y;
      }

      if is_mouse_button_released(MouseButton::Left) {
        mouse_scale_mode = false;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text(
      "Scale rectangle dragging from bottom-right corner!",
      10,
      10,
      20,
      colors::GRAY,
    );

    draw_rectangle_rec(rec, fade(colors::GREEN, 0.5));

    if mouse_scale_ready {
      draw_rectangle_lines_ex(rec, 1.0, colors::RED);
      draw_triangle(
        Vector2 {
          x: rec.x + rec.width - MOUSE_SCALE_MARK_SIZE as f32,
          y: rec.y + rec.height,
        },
        Vector2 {
          x: rec.x + rec.width,
          y: rec.y + rec.height,
        },
        Vector2 {
          x: rec.x + rec.width,
          y: rec.y + rec.height - MOUSE_SCALE_MARK_SIZE as f32,
        },
        colors::RED,
      );
    }

    end_drawing();
  }
  close_window();
}
