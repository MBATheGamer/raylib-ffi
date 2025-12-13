use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    get_screen_height, get_screen_to_world_2d, get_screen_width, init_window,
    keyboard::is_key_pressed,
    mouse::{
      get_mouse_delta, get_mouse_position, get_mouse_wheel_move, get_mouse_x, get_mouse_y,
      is_mouse_button_down, is_mouse_button_pressed,
    },
    set_target_fps, window_should_close,
  },
  enums::{KeyboardKey, MouseButton},
  model::draw_grid,
  rlgl::{rl_pop_matrix, rl_push_matrix, rl_rotate, rl_translate},
  shape::{draw_circle, draw_circle_v},
  structs::{Camera2D, Vector2},
  text::{draw_text, draw_text_ex, get_font_default},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 2d camera mouse zoom",
  );

  let mut camera = Camera2D {
    zoom: 1.0,
    ..Camera2D::default()
  };

  let mut zoom_mode = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyOne) {
      zoom_mode = 0;
    } else if is_key_pressed(KeyboardKey::KeyTwo) {
      zoom_mode = 1;
    }

    if is_mouse_button_down(MouseButton::Left) {
      let mut delta = get_mouse_delta();
      delta = delta.scale(-1.0 / camera.zoom);
      camera.target = camera.target + delta;
    }

    if zoom_mode == 0 {
      let wheel = get_mouse_wheel_move();
      if wheel != 0.0 {
        let mouse_world_pos = get_screen_to_world_2d(get_mouse_position(), camera);

        camera.offset = get_mouse_position();

        camera.target = mouse_world_pos;

        let scale = 0.2 * wheel;
        camera.zoom = (camera.zoom.ln() + scale).exp().clamp(0.125, 64.0);
      }
    } else {
      if is_mouse_button_pressed(MouseButton::Right) {
        let mouse_world_pos = get_screen_to_world_2d(get_mouse_position(), camera);

        camera.offset = get_mouse_position();

        camera.target = mouse_world_pos;
      }

      if is_mouse_button_down(MouseButton::Right) {
        let delta_x = get_mouse_delta().x;
        let scale = 0.005 * delta_x;
        camera.zoom = (camera.zoom.ln() + scale).exp().clamp(0.125, 64.0);
      }
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    begin_mode_2d(camera);

    rl_push_matrix();
    rl_translate(0.0, 25.0 * 50.0, 0.0);
    rl_rotate(90.0, 1.0, 0.0, 0.0);
    draw_grid(100, 50.0);
    rl_pop_matrix();

    draw_circle(
      get_screen_width() / 2,
      get_screen_height() / 2,
      50.0,
      colors::MAROON,
    );
    end_mode_2d();

    draw_circle_v(get_mouse_position(), 4.0, colors::DARKGRAY);
    draw_text_ex(
      get_font_default(),
      &format!("[{}, {}]", get_mouse_x(), get_mouse_y()),
      get_mouse_position() + Vector2 { x: -44.0, y: -24.0 },
      20.0,
      2.0,
      colors::BLACK,
    );

    draw_text(
      "[1][2] Select mouse zoom mode (Wheel or Move)",
      20,
      20,
      20,
      colors::DARKGRAY,
    );
    if zoom_mode == 0 {
      draw_text(
        "Mouse left button drag to move, mouse wheel to zoom",
        20,
        50,
        20,
        colors::DARKGRAY,
      );
    } else {
      draw_text(
        "Mouse left button drag to move, mouse press and move to zoom",
        20,
        50,
        20,
        colors::DARKGRAY,
      );
    }

    end_drawing();
  }

  close_window();
}
