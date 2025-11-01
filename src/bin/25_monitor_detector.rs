use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_current_monitor,
    get_monitor_count, get_monitor_height, get_monitor_name, get_monitor_physical_height,
    get_monitor_physical_width, get_monitor_position, get_monitor_refresh_rate, get_monitor_width,
    get_window_position, init_window, keyboard::is_key_pressed, set_target_fps, set_window_monitor,
    window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_rectangle_lines, draw_rectangle_lines_ex, draw_rectangle_v},
  structs::{Rectangle, Vector2},
  text::draw_text,
  texture::fade,
};

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

  let mut monitors: [MonitorInfo; MAX_MONITORS] = [Default::default(); MAX_MONITORS];

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - monitor detector",
  );

  let mut current_monitor_index = get_current_monitor();

  set_target_fps(60);

  while !window_should_close() {
    let mut max_width = 1;
    let mut max_height = 1;

    let mut monitor_offset_x = 0;

    let monitor_count = get_monitor_count();
    for i in 0..monitor_count as usize {
      monitors[i] = MonitorInfo {
        position: get_monitor_position(i as i32),
        name: get_monitor_name(i as i32),
        width: get_monitor_width(i as i32),
        height: get_monitor_height(i as i32),
        physical_width: get_monitor_physical_width(i as i32),
        physical_height: get_monitor_physical_height(i as i32),
        refresh_rate: get_monitor_refresh_rate(i as i32),
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

    let mut monitor_scale = 0.6;

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

    for i in 0..monitor_count as usize {
      let rec = Rectangle {
        x: (monitors[i].position.x + monitor_offset_x as f32) * monitor_scale + 140.0,
        y: monitors[i].position.y * monitor_scale + 80.0,
        width: monitors[i].width as f32 * monitor_scale,
        height: monitors[i].height as f32 * monitor_scale,
      };

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

      if i == current_monitor_index as usize {
        draw_rectangle_lines_ex(rec, 5.0, colors::RED);
        let window_position = Vector2 {
          x: (get_window_position().x + monitor_offset_x as f32) * monitor_scale + 140.0,
          y: get_window_position().y * monitor_scale + 80.0,
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
        draw_rectangle_lines_ex(rec, 5.0, colors::GRAY);
      }
    }

    end_drawing();
  }

  close_window();
}
