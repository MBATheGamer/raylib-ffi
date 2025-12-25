use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode, init_window,
    mouse::{
      get_mouse_position, get_mouse_wheel_move, is_mouse_button_down, is_mouse_button_pressed,
    },
    set_target_fps, window_should_close,
  },
  enums::MouseButton,
  shape::{draw_circle_lines_v, draw_circle_v, draw_line_ex},
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
  texture::{color_from_hsv, draw_texture_rec, load_render_texture, unload_render_texture},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - lines drawing",
  );

  let mut start_text = true;

  let mut mouse_position_previous = get_mouse_position();

  let canvas = load_render_texture(SCREEN_WIDTH, SCREEN_HEIGHT);

  let mut line_thickness = 8.0;

  let mut line_hue = 0.0;

  begin_texture_mode(canvas);
  clear_background(colors::RAYWHITE);
  end_texture_mode();

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_pressed(MouseButton::Left) && start_text {
      start_text = false;
    }

    if is_mouse_button_pressed(MouseButton::Middle) {
      begin_texture_mode(canvas);
      clear_background(colors::RAYWHITE);
      end_texture_mode();
    }

    let left_button_down = is_mouse_button_down(MouseButton::Left);
    let right_button_down = is_mouse_button_down(MouseButton::Right);

    if left_button_down || right_button_down {
      let mut draw_color = colors::WHITE;

      if left_button_down {
        line_hue += mouse_position_previous.distance(get_mouse_position()) / 3.0;

        while line_hue >= 360.0 {
          line_hue -= 360.0;
        }

        draw_color = color_from_hsv(line_hue, 1.0, 1.0);
      } else if right_button_down {
        draw_color = colors::RAYWHITE;
      }

      begin_texture_mode(canvas);

      draw_circle_v(mouse_position_previous, line_thickness / 2.0, draw_color);
      draw_circle_v(get_mouse_position(), line_thickness / 2.0, draw_color);
      draw_line_ex(
        mouse_position_previous,
        get_mouse_position(),
        line_thickness,
        draw_color,
      );
      end_texture_mode();
    }

    line_thickness += get_mouse_wheel_move();
    line_thickness = line_thickness.clamp(1.0, 500.0);

    mouse_position_previous = get_mouse_position();

    begin_drawing();

    draw_texture_rec(
      canvas.texture,
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: canvas.texture.width as f32,
        height: -canvas.texture.height as f32,
      },
      Vector2::zero(),
      colors::WHITE,
    );

    if !left_button_down {
      draw_circle_lines_v(
        get_mouse_position(),
        line_thickness / 2.0,
        Color {
          red: 127,
          green: 127,
          blue: 127,
          alpha: 127,
        },
      );
    }

    if start_text {
      draw_text(
        "try clicking and dragging!",
        275,
        215,
        20,
        colors::LIGHTGRAY,
      );
    }

    end_drawing();
  }

  unload_render_texture(canvas);

  close_window();
}
