use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    keyboard::is_key_pressed,
    mouse::{get_mouse_position, is_mouse_button_down},
    set_target_fps, window_should_close,
  },
  enums::{KeyboardKey, MouseButton},
  shape::{draw_circle_sector, draw_line, draw_line_ex},
  structs::Vector2,
  text::draw_text,
  texture::fade,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - vector angle",
  );

  let v0 = Vector2 {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
  };
  let mut v1 = v0 + Vector2 { x: 100.0, y: 80.0 };

  let mut angle = 0.0;
  let mut angle_mode = false;

  set_target_fps(60);

  while !window_should_close() {
    let startangle = if !angle_mode {
      (-v0.line_angle(v1)).to_degrees()
    } else {
      0.0
    };

    let v2 = get_mouse_position();

    if is_key_pressed(KeyboardKey::KeySpace) {
      angle_mode = !angle_mode;
    }

    if !angle_mode && is_mouse_button_down(MouseButton::Right) {
      v1 = get_mouse_position();
    }

    if !angle_mode {
      let v1_normal = (v1 - v0).normalize();
      let v2_normal = (v2 - v0).normalize();

      angle = v1_normal.angle(v2_normal).to_degrees();
    } else if angle_mode {
      angle = v0.line_angle(v2).to_degrees();
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if !angle_mode {
      draw_text("MODE 0: Angle between V1 and V2", 10, 10, 20, colors::BLACK);
      draw_text("Right Click to Move V2", 10, 30, 20, colors::DARKGRAY);

      draw_line_ex(v0, v1, 2.0, colors::BLACK);
      draw_line_ex(v0, v2, 2.0, colors::RED);

      draw_circle_sector(
        v0,
        40.0,
        startangle,
        startangle + angle,
        32,
        fade(colors::GREEN, 0.6),
      );
    } else if angle_mode {
      draw_text(
        "MODE 1: Angle formed by line V1 to V2",
        10,
        10,
        20,
        colors::BLACK,
      );

      draw_line(
        0,
        SCREEN_HEIGHT / 2,
        SCREEN_WIDTH,
        SCREEN_HEIGHT / 2,
        colors::LIGHTGRAY,
      );
      draw_line_ex(v0, v2, 2.0, colors::RED);

      draw_circle_sector(
        v0,
        40.0,
        startangle,
        startangle - angle,
        32,
        fade(colors::GREEN, 0.6),
      );
    }

    draw_text("v0", v0.x as i32, v0.y as i32, 10, colors::DARKGRAY);

    if !angle_mode && (v0 - v1).y > 0.0 {
      draw_text("v1", v1.x as i32, v1.y as i32 - 10, 10, colors::DARKGRAY);
    }
    if !angle_mode && (v0 - v1).y < 0.0 {
      draw_text("v1", v1.x as i32, v1.y as i32, 10, colors::DARKGRAY);
    }

    if angle_mode {
      draw_text("v1", v0.x as i32 + 40, v0.y as i32, 10, colors::DARKGRAY);
    }

    draw_text(
      "v2",
      v2.x as i32 - 10,
      v2.y as i32 - 10,
      10,
      colors::DARKGRAY,
    );

    draw_text("Press SPACE to change MODE", 460, 10, 20, colors::DARKGRAY);
    draw_text(&format!("ANGLE: {:2.2}", angle), 10, 70, 20, colors::LIME);

    end_drawing();
  }

  close_window();
}
