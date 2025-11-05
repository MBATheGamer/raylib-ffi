fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::WindowHighDPI, ConfigFlags::WindowResizable]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - high dpi",
  );
  set_window_min_size(450, 450);

  let logical_grid_desc_y = 120;
  let logical_grid_label_y = logical_grid_desc_y + 30;
  let logical_grid_top = logical_grid_label_y + 30;
  let logical_grid_bottom = logical_grid_top + 80;
  let pixel_grid_top = logical_grid_bottom - 20;
  let pixel_grid_bottom = pixel_grid_top + 80;
  let pixel_grid_label_y = pixel_grid_bottom + 30;
  let pixel_grid_desc_y = pixel_grid_label_y + 30;
  let cell_size = 50;
  let cell_size_px = 50.0;

  set_target_fps(60);

  while !window_should_close() {
    let monitor_count = get_monitor_count();

    if monitor_count > 1 && is_key_pressed(KeyboardKey::KeyN) {
      set_window_monitor((get_current_monitor() + 1) % monitor_count);
    }

    let current_monitor = get_current_monitor();
    let dpi_scale = get_window_scale_dpi();
    cell_size_px = cell_size as f32 / dpi_scale.x;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    let window_center = get_screen_width() / 2;
    draw_text_center(
      &format!("Dpi Scale: {}", dpi_scale.x),
      window_center,
      30,
      40,
      colors::DARKGRAY,
    );
    draw_text_center(
      &format!(
        "Monitor: {}/{} ([N] next monitor)",
        current_monitor + 1,
        monitor_count
      ),
      window_center,
      70,
      20,
      colors::LIGHTGRAY,
    );
    draw_text_center(
      &format!("Window is {} \"logical points\" wide", get_screen_width()),
      window_center,
      logical_grid_desc_y,
      20,
      colors::ORANGE,
    );

    let odd = true;
    for i in cell_size..get_screen_width() {
      if odd {
        draw_rectangle(
          i,
          logical_grid_top,
          cell_size,
          logical_grid_bottom - logical_grid_top,
          colors::ORANGE,
        );
      }
      draw_text_center(
        &format!("{}", i),
        i,
        logical_grid_label_y,
        10,
        colors::LIGHTGRAY,
      );
      draw_line(
        i,
        logical_grid_label_y + 10,
        i,
        logical_grid_bottom,
        colors::GRAY,
      );
      i += cell_size;
      odd = !odd;
    }

    odd = true;
    let min_text_space = 30;
    let last_text_x = -min_text_space;
    for i in cell_size..get_render_width() {
      let x = (i / dpi_scale.x) as i32;
      if odd {
        draw_rectangle(
          x,
          pixel_grid_top,
          cell_size_px as i32,
          pixel_grid_bottom - pixel_grid_top,
          Color {
            red: 0,
            green: 121,
            blue: 241,
            alpha: 100,
          },
        );
      }
      draw_line(
        x,
        pixel_grid_top,
        (i as f32 / dpi_scale.x) as i32,
        pixel_grid_label_y - 10,
        colors::GRAY,
      );

      if (x - last_text_x) >= min_text_space {
        draw_text_center(
          &format!("{}", i),
          x,
          pixel_grid_label_y,
          10,
          colors::LIGHTGRAY,
        );
        last_text_x = x;
      }

      i += cell_size;
      odd = !odd;
    }

    draw_text_center(
      &format!("Window is {} \"physical pixels\" wide", get_render_width()),
      window_center,
      pixel_grid_desc_y,
      20,
      colors::BLUE,
    );

    let text = "Can you see this?";
    let size = measure_text_ex(get_font_default(), text, 20, 3);
    let pos = Vector2 {
      x: get_screen_width() as f32 - size.x - 5.0,
      y: get_screen_height() as f32 - size.y - 5.0,
    };
    draw_text_ex(get_font_default(), text, pos, 20, 3, colors::LIGHTGRAY);

    end_drawing();
  }

  close_window();
}

fn draw_text_center(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
  let size = measure_text_ex(get_font_default(), text, font_size, 3);
  let pos = Vector2 {
    x: x as f32 - size.x / 2.0,
    y: y as f32 - size.y / 2.0,
  };
  draw_text_ex(get_font_default(), text, pos, font_size, 3, color);
}
