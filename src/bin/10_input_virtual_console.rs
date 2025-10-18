use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_frame_time, init_window,
    mouse::{get_mouse_position, is_mouse_button_down},
    set_target_fps,
    touch::{get_touch_point_count, get_touch_position},
    window_should_close,
  },
  enums::MouseButton,
  shapes::draw_circle_v,
  structs::{Color, Vector2},
  text::draw_text,
};

#[derive(Clone, Copy)]
enum PadButton {
  None = -1,
  Up = 0,
  Left = 1,
  Right = 2,
  Down = 3,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  const MAX_BUTTON: usize = 4;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input virtual controls",
  );

  let pad_position = Vector2 { x: 100.0, y: 350.0 };
  let button_radius: f32 = 30.0;

  let button_positions: Vec<Vector2> = vec![
    Vector2 {
      x: pad_position.x,
      y: pad_position.y - button_radius * 1.5,
    },
    Vector2 {
      x: pad_position.x - button_radius * 1.5,
      y: pad_position.y,
    },
    Vector2 {
      x: pad_position.x + button_radius * 1.5,
      y: pad_position.y,
    },
    Vector2 {
      x: pad_position.x,
      y: pad_position.y + button_radius * 1.5,
    },
  ];

  let button_labels: Vec<&str> = vec!["Y", "X", "B", "A"];

  let button_label_colors: Vec<Color> =
    vec![colors::YELLOW, colors::BLUE, colors::RED, colors::GREEN];

  let mut player_position = Vector2 {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
  };
  let player_speed = 75.0;

  set_target_fps(60);

  while !window_should_close() {
    let input_position: Vector2;

    if get_touch_point_count() > 0 {
      input_position = get_touch_position(0);
    } else {
      input_position = get_mouse_position();
    }

    let mut pressed_button: PadButton = PadButton::None;

    if (get_touch_point_count() > 0)
      || ((get_touch_point_count() == 0) && is_mouse_button_down(MouseButton::Left))
    {
      for i in 0..MAX_BUTTON {
        let dist_x = (button_positions[i as usize].x - input_position.x).abs();
        let dist_y = (button_positions[i as usize].y - input_position.y).abs();

        if dist_x + dist_y < button_radius {
          pressed_button = match i {
            0 => PadButton::Up,
            1 => PadButton::Left,
            2 => PadButton::Right,
            3 => PadButton::Down,
            _ => PadButton::None,
          };
          break;
        }
      }
    }

    match pressed_button {
      PadButton::Up => player_position.y -= player_speed * get_frame_time(),
      PadButton::Left => player_position.x -= player_speed * get_frame_time(),
      PadButton::Right => player_position.x += player_speed * get_frame_time(),
      PadButton::Down => player_position.y += player_speed * get_frame_time(),
      _ => {}
    };

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_circle_v(player_position, 50.0, colors::MAROON);

    for i in 0..MAX_BUTTON as usize {
      draw_circle_v(
        button_positions[i],
        button_radius,
        if i == pressed_button as usize {
          colors::DARKGRAY
        } else {
          colors::BLACK
        },
      );

      draw_text(
        button_labels[i],
        button_positions[i].x as i32 - 7,
        button_positions[i].y as i32 - 8,
        20,
        button_label_colors[i],
      );
    }

    draw_text(
      "move the player with D-Pad buttons",
      10,
      10,
      20,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  close_window();
}
