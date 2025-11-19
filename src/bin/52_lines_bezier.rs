use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    mouse::{get_mouse_position, is_mouse_button_down, is_mouse_button_released},
    set_config_flags, set_target_fps, window_should_close,
  },
  enums::{ConfigFlags, MouseButton},
  shape::{check_collision_point_circle, draw_circle_v, draw_line_bezier},
  structs::Vector2,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::MSAA4xHint]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - lines bezier",
  );

  let mut start_point = Vector2 { x: 30.0, y: 30.0 };
  let mut end_point = Vector2 {
    x: SCREEN_WIDTH as f32 - 30.0,
    y: SCREEN_HEIGHT as f32 - 30.0,
  };
  let mut move_start_point = false;
  let mut move_end_point = false;

  set_target_fps(60);

  while !window_should_close() {
    let mouse = get_mouse_position();

    if check_collision_point_circle(mouse, start_point, 10.0)
      && is_mouse_button_down(MouseButton::Left)
    {
      move_start_point = true;
    } else if check_collision_point_circle(mouse, end_point, 10.0)
      && is_mouse_button_down(MouseButton::Left)
    {
      move_end_point = true;
    }

    if move_start_point {
      start_point = mouse;
      if is_mouse_button_released(MouseButton::Left) {
        move_start_point = false;
      }
    }

    if move_end_point {
      end_point = mouse;
      if is_mouse_button_released(MouseButton::Left) {
        move_end_point = false;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text("MOVE START-END POINTS WITH MOUSE", 15, 20, 20, colors::GRAY);

    draw_line_bezier(start_point, end_point, 4.0, colors::BLUE);

    draw_circle_v(
      start_point,
      if check_collision_point_circle(mouse, start_point, 10.0) {
        14.0
      } else {
        8.0
      },
      if move_start_point {
        colors::RED
      } else {
        colors::BLUE
      },
    );
    draw_circle_v(
      end_point,
      if check_collision_point_circle(mouse, end_point, 10.0) {
        14.0
      } else {
        8.0
      },
      if move_end_point {
        colors::RED
      } else {
        colors::BLUE
      },
    );

    end_drawing();
  }

  close_window();
}
