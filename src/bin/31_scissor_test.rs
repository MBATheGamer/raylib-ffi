use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_scissor_mode, clear_background, close_window, end_drawing,
    end_scissor_mode, get_screen_height, get_screen_width, init_window,
    keyboard::is_key_pressed,
    mouse::{get_mouse_x, get_mouse_y},
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_rectangle, draw_rectangle_lines_ex},
  structs::Rectangle,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - scissor test",
  );

  let mut scissor_area = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 300.0,
    height: 300.0,
  };
  let mut scissor_mode = true;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyS) {
      scissor_mode = !scissor_mode;
    }

    scissor_area.x = get_mouse_x() as f32 - scissor_area.width / 2.0;
    scissor_area.y = get_mouse_y() as f32 - scissor_area.height / 2.0;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if scissor_mode {
      begin_scissor_mode(
        scissor_area.x as i32,
        scissor_area.y as i32,
        scissor_area.width as i32,
        scissor_area.height as i32,
      );
    }

    draw_rectangle(0, 0, get_screen_width(), get_screen_height(), colors::RED);
    draw_text(
      "Move the mouse around to reveal this text!",
      190,
      200,
      20,
      colors::LIGHTGRAY,
    );

    if scissor_mode {
      end_scissor_mode();
    }

    draw_rectangle_lines_ex(scissor_area, 1.0, colors::BLACK);
    draw_text("Press S to toggle scissor test", 10, 10, 20, colors::BLACK);

    end_drawing();
  }

  close_window();
}
