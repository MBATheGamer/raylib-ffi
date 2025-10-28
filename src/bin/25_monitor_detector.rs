#[derive(Clone, Copy, Default)]
struct MonitorInfo {
  position: Vector2,
  name: &'static str,
  width: i32,
  height: i32,
  physical_width: i32,
  physical_height: i32,
  refresh_rate: i32,
}

fn main() {
  const MAX_MONITORS: usize = 10;

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  let monitors: [MonitorInfo; MAX_MONITORS] = [Default::default(); MAX_MONITORS];

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - monitor detector",
  );

  let current_monitor_index = get_current_monitor();
  let monitor_count = 0;

  set_target_fps(60);

  while !window_should_close() {
    let max_width = 1;
    let max_height = 1;

    let monitor_offset_x = 0;

    monitor_count = get_monitor_count();
    for i in 0..monitor_count {
      monitors[i] = MonitorInfo {
        position: get_monitor_position(i),
        name: get_monitor_name(i),
        width: get_monitor_width(i),
        height: get_monitor_height(i),
        physical_width: get_monitor_physical_width(i),
        physical_height: get_monitor_physical_height(i),
        refresh_rate: get_monitor_refresh_rate(i),
      };

      if (monitors[i].position.x as i32) < monitor_offset_x {
        monitor_offset_x = monitors[i].position.x as i32 * -1;
      }

      let width = monitors[i].position.x as i32 + monitors[i].width;
      let height = monitors[i].position.y as i32 + monitors[i].height;

      if max_width < width {
        max_width = width;
      }
      if max_height < height {
        max_height = height;
      }
    }

    if is_key_pressed(KeyboardKey::KeyEnter) && monitor_count > 1 {
      current_monitor_index += 1;

      if current_monitor_index == monitor_count {
        current_monitor_index = 0;
      }

      set_window_monitor(current_monitor_index);
    } else {
      current_monitor_index = get_current_monitor();
    }

    let monitor_scale = 0.6;

    if max_height > max_width + monitor_offset_x {
      monitor_scale *= SCREEN_HEIGHT as f32 / max_height as f32;
    } else {
      monitor_scale *= SCREEN_WIDTH as f32 / (max_width + monitor_offset_x) as f32;
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_text(
      "Press [Enter] to move window to next monitor available",
      20,
      20,
      20,
      colors::DARKGRAY,
    );

    draw_rectangle_lines(
      20,
      60,
      SCREEN_WIDTH - 40,
      SCREEN_HEIGHT - 100,
      colors::DARKGRAY,
    );

    for i in 0..monitor_count {
      let rec = Rectangle {
        x: (monitors[i].position.x + monitor_offset_x as f32) * monitor_scale + 140.0,
        y: monitors[i].position.y * monitor_scale + 80.0,
        width: monitors[i].width * monitor_scale as f32,
        height: monitors[i].height * monitor_scale as f32,
      };

      // Draw monitor name and information inside the rectangle
      draw_text(
        &format!("[{}] {}", i, monitors[i].name),
        rec.x as i32 + 10,
        rec.y as i32 + (100.0 * monitor_scale) as i32,
        (120.0 * monitor_scale) as i32,
        colors::BLUE,
      );
      draw_text(
        &format!(
          "Resolution: [{}px x {}px]\nRefreshRate: [{}hz]\nPhysical Size: [{}mm x {}mm]\nPosition: {} x {}",
          monitors[i].width,
          monitors[i].height,
          monitors[i].refresh_rate,
          monitors[i].physical_width,
          monitors[i].physical_height,
          monitors[i].position.x,
          monitors[i].position.y
        ),
        rec.x as i32 + 10,
        rec.y as i32 + (200.0 * monitor_scale) as i32,
        (120.0 * monitor_scale) as i32,
        colors::DARKGRAY,
      );

      if i == current_monitor_index {
        draw_rectangle_linesEx(rec, 5, colors::RED);
        let window_position = Vector2 {
          x: (get_window_position().x + monitor_offset_x) * monitor_scale + 140,
          y: get_window_position().y * monitor_scale + 80,
        };

        draw_rectangle_v(
          window_position,
          Vector2 {
            x: SCREEN_WIDTH as f32 * monitor_scale,
            y: SCREEN_HEIGHT as f32 * monitor_scale,
          },
          fade(colors::GREEN, 0.5),
        );
      } else {
        draw_rectangle_linesEx(rec, 5, colors::GRAY);
      }
    }

    end_drawing();
  }

  close_window();
}
