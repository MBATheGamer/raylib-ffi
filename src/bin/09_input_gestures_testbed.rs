use std::f32::consts::PI;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing,
    gesture::{get_gesture_detected, get_gesture_drag_angle, get_gesture_pinch_angle},
    init_window,
    mouse::{get_mouse_position, is_mouse_button_released},
    set_target_fps,
    touch::{get_touch_point_count, get_touch_position},
    window_should_close,
  },
  enums::{Gesture, MouseButton},
  shape::{
    check_collision_point_rec, draw_circle, draw_circle_v, draw_line_ex, draw_rectangle,
    draw_rectangle_rec, draw_ring, draw_triangle,
  },
  structs::{Color, Rectangle, Vector2},
  text::draw_text,
  texture::fade,
};

fn main() {
  const GESTURE_LOG_SIZE: i32 = 20;

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input gestures testbed",
  );

  let message_position = Vector2 { x: 160.0, y: 7.0 };

  let mut last_gesture = Gesture::None;
  let last_gesture_position = Vector2 { x: 165.0, y: 130.0 };

  let mut gesture_log: Vec<&str> = vec![
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
  ];

  let mut gesture_log_index = GESTURE_LOG_SIZE - 1;
  let mut previous_gesture = Gesture::None;

  let mut log_mode = 1;

  let mut gesture_color = Color {
    red: 0,
    green: 0,
    blue: 0,
    alpha: 255,
  };
  let log_button1 = Rectangle {
    x: 53.0,
    y: 7.0,
    width: 48.0,
    height: 26.0,
  };
  let log_button2 = Rectangle {
    x: 108.0,
    y: 7.0,
    width: 36.0,
    height: 26.0,
  };
  let gesture_log_position = Vector2 { x: 10.0, y: 10.0 };

  let angle_length = 90.0;
  let mut current_angle_degrees = 0.0;
  let protractor_position = Vector2 { x: 266.0, y: 315.0 };

  set_target_fps(60);

  while !window_should_close() {
    let current_gesture: Gesture = get_gesture_detected();
    let current_drag_degrees: f32 = get_gesture_drag_angle();
    let current_pitch_degrees: f32 = get_gesture_pinch_angle();
    let touch_count: i32 = get_touch_point_count();

    if current_gesture != Gesture::None
      && current_gesture != Gesture::Hold
      && current_gesture != previous_gesture
    {
      last_gesture = current_gesture;
    }

    if is_mouse_button_released(MouseButton::Left) {
      if check_collision_point_rec(get_mouse_position(), log_button1) {
        match log_mode {
          3 => log_mode = 2,
          2 => log_mode = 3,
          1 => log_mode = 0,
          _ => log_mode = 1,
        }
      } else if check_collision_point_rec(get_mouse_position(), log_button2) {
        match log_mode {
          3 => log_mode = 1,
          2 => log_mode = 0,
          1 => log_mode = 3,
          _ => log_mode = 2,
        }
      }
    }

    let mut fill_log = 0;
    if current_gesture != Gesture::None {
      if log_mode == 3 {
        if current_gesture != Gesture::Hold && current_gesture != previous_gesture
          || current_gesture < Gesture::Hold
        {
          fill_log = 1;
        }
      } else if log_mode == 2 {
        if current_gesture != Gesture::Hold {
          fill_log = 1;
        }
      } else if log_mode == 1 {
        if current_gesture != previous_gesture {
          fill_log = 1;
        }
      } else {
        fill_log = 1;
      }
    }

    if fill_log != 0 {
      previous_gesture = current_gesture;
      gesture_color = get_gesture_color(current_gesture);
      if gesture_log_index <= 0 {
        gesture_log_index = GESTURE_LOG_SIZE;
      }
      gesture_log_index -= 1;

      gesture_log[gesture_log_index as usize] = get_gesture_name(current_gesture);
    }

    if current_gesture > Gesture::SwipeDown {
      current_angle_degrees = current_pitch_degrees;
    } else if current_gesture > Gesture::Drag {
      current_angle_degrees = current_drag_degrees;
    } else if current_gesture > Gesture::None {
      current_angle_degrees = 0.0;
    }

    let current_angle_radians: f32 = (current_angle_degrees + 90.0) * PI / 180.0;

    let final_vector = Vector2 {
      x: (angle_length * current_angle_radians.sin()) + protractor_position.x,
      y: (angle_length * current_angle_radians.cos()) + protractor_position.y,
    };

    let mut touch_position: Vec<Vector2> = vec![];
    let mut mouse_position: Vector2 = Vector2::default();

    if current_gesture != Gesture::None {
      if touch_count != 0 {
        for i in 0..touch_count {
          touch_position.push(get_touch_position(i));
        }
      } else {
        mouse_position = get_mouse_position();
      }
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_text(
      "*",
      message_position.x as i32 + 5,
      message_position.y as i32 + 5,
      10,
      colors::BLACK,
    );
    draw_text(
      "Example optimized for Web/HTML5\non Smartphones with Touch Screen.",
      message_position.x as i32 + 15,
      message_position.y as i32 + 5,
      10,
      colors::BLACK,
    );
    draw_text(
      "*",
      message_position.x as i32 + 5,
      message_position.y as i32 + 35,
      10,
      colors::BLACK,
    );
    draw_text(
      "While running on Desktop Web Browsers,\ninspect and turn on Touch Emulation.",
      message_position.x as i32 + 15,
      message_position.y as i32 + 35,
      10,
      colors::BLACK,
    );

    draw_text(
      "Last gesture",
      last_gesture_position.x as i32 + 33,
      last_gesture_position.y as i32 - 47,
      20,
      colors::BLACK,
    );
    draw_text(
      "Swipe         Tap       Pinch  Touch",
      last_gesture_position.x as i32 + 17,
      last_gesture_position.y as i32 - 18,
      10,
      colors::BLACK,
    );
    draw_rectangle(
      last_gesture_position.x as i32 + 20,
      last_gesture_position.y as i32,
      20,
      20,
      if last_gesture == Gesture::SwipeUp {
        colors::RED
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_rectangle(
      last_gesture_position.x as i32,
      last_gesture_position.y as i32 + 20,
      20,
      20,
      if last_gesture == Gesture::SwipeLeft {
        colors::RED
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_rectangle(
      last_gesture_position.x as i32 + 40,
      last_gesture_position.y as i32 + 20,
      20,
      20,
      if last_gesture == Gesture::SwipeRight {
        colors::RED
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_rectangle(
      last_gesture_position.x as i32 + 20,
      last_gesture_position.y as i32 + 40,
      20,
      20,
      if last_gesture == Gesture::SwipeDown {
        colors::RED
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_circle(
      last_gesture_position.x as i32 + 80,
      last_gesture_position.y as i32 + 16,
      10.0,
      if last_gesture == Gesture::Tap {
        colors::BLUE
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_ring(
      Vector2 {
        x: last_gesture_position.x + 103.0,
        y: last_gesture_position.y + 16.0,
      },
      6.0,
      11.0,
      0.0,
      360.0,
      0,
      if last_gesture == Gesture::Drag {
        colors::LIME
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_circle(
      last_gesture_position.x as i32 + 80,
      last_gesture_position.y as i32 + 43,
      10.0,
      if last_gesture == Gesture::DoubleTap {
        colors::SKYBLUE
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_circle(
      last_gesture_position.x as i32 + 103,
      last_gesture_position.y as i32 + 43,
      10.0,
      if last_gesture == Gesture::DoubleTap {
        colors::SKYBLUE
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_triangle(
      Vector2 {
        x: last_gesture_position.x + 122.0,
        y: last_gesture_position.y + 16.0,
      },
      Vector2 {
        x: last_gesture_position.x + 137.0,
        y: last_gesture_position.y + 26.0,
      },
      Vector2 {
        x: last_gesture_position.x + 137.0,
        y: last_gesture_position.y + 6.0,
      },
      if last_gesture == Gesture::PinchOut {
        colors::ORANGE
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_triangle(
      Vector2 {
        x: last_gesture_position.x + 147.0,
        y: last_gesture_position.y + 6.0,
      },
      Vector2 {
        x: last_gesture_position.x + 147.0,
        y: last_gesture_position.y + 26.0,
      },
      Vector2 {
        x: last_gesture_position.x + 162.0,
        y: last_gesture_position.y + 16.0,
      },
      if last_gesture == Gesture::PinchOut {
        colors::ORANGE
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_triangle(
      Vector2 {
        x: last_gesture_position.x + 125.0,
        y: last_gesture_position.y + 33.0,
      },
      Vector2 {
        x: last_gesture_position.x + 125.0,
        y: last_gesture_position.y + 53.0,
      },
      Vector2 {
        x: last_gesture_position.x + 140.0,
        y: last_gesture_position.y + 43.0,
      },
      if last_gesture == Gesture::PinchIn {
        colors::VIOLET
      } else {
        colors::LIGHTGRAY
      },
    );
    draw_triangle(
      Vector2 {
        x: last_gesture_position.x + 144.0,
        y: last_gesture_position.y + 43.0,
      },
      Vector2 {
        x: last_gesture_position.x + 159.0,
        y: last_gesture_position.y + 53.0,
      },
      Vector2 {
        x: last_gesture_position.x + 159.0,
        y: last_gesture_position.y + 33.0,
      },
      if last_gesture == Gesture::PinchIn {
        colors::VIOLET
      } else {
        colors::LIGHTGRAY
      },
    );
    for i in 0..4 {
      draw_circle(
        last_gesture_position.x as i32 + 180,
        last_gesture_position.y as i32 + 7 + i * 15,
        5.0,
        if touch_count <= i {
          colors::LIGHTGRAY
        } else {
          gesture_color
        },
      );
    }

    draw_text(
      "Log",
      gesture_log_position.x as i32,
      gesture_log_position.y as i32,
      20,
      colors::BLACK,
    );

    let mut ii = gesture_log_index;
    for i in 0..GESTURE_LOG_SIZE {
      draw_text(
        gesture_log[ii as usize],
        gesture_log_position.x as i32,
        gesture_log_position.y as i32 + 410 - i * 20,
        20,
        if i == 0 {
          gesture_color
        } else {
          colors::LIGHTGRAY
        },
      );

      ii = (ii + 1) % GESTURE_LOG_SIZE;
    }
    let log_button1_color: Color;
    let log_button2_color: Color;
    match log_mode {
      3 => {
        log_button1_color = colors::MAROON;
        log_button2_color = colors::MAROON;
      }
      2 => {
        log_button1_color = colors::GRAY;
        log_button2_color = colors::MAROON;
      }
      1 => {
        log_button1_color = colors::MAROON;
        log_button2_color = colors::GRAY;
      }
      _ => {
        log_button1_color = colors::GRAY;
        log_button2_color = colors::GRAY;
      }
    }
    draw_rectangle_rec(log_button1, log_button1_color);
    draw_text(
      "Hide",
      log_button1.x as i32 + 7,
      log_button1.y as i32 + 3,
      10,
      colors::WHITE,
    );
    draw_text(
      "Repeat",
      log_button1.x as i32 + 7,
      log_button1.y as i32 + 13,
      10,
      colors::WHITE,
    );
    draw_rectangle_rec(log_button2, log_button2_color);
    draw_text(
      "Hide",
      log_button1.x as i32 + 62,
      log_button1.y as i32 + 3,
      10,
      colors::WHITE,
    );
    draw_text(
      "Hold",
      log_button1.x as i32 + 62,
      log_button1.y as i32 + 13,
      10,
      colors::WHITE,
    );

    draw_text(
      "Angle",
      protractor_position.x as i32 + 55,
      protractor_position.y as i32 + 76,
      10,
      colors::BLACK,
    );
    let angle_string: &str = &format!("{}", current_angle_degrees);
    let angle_string_dot = angle_string.find(".");
    match angle_string_dot {
      Some(index) => {
        let angle_string_trim: &str = &angle_string[0..index + 3];
        draw_text(
          angle_string_trim,
          protractor_position.x as i32 + 55,
          protractor_position.y as i32 + 92,
          20,
          gesture_color,
        );
      }
      None => {}
    }
    draw_circle_v(protractor_position, 80.0, colors::WHITE);
    draw_line_ex(
      Vector2 {
        x: protractor_position.x - 90.0,
        y: protractor_position.y,
      },
      Vector2 {
        x: protractor_position.x + 90.0,
        y: protractor_position.y,
      },
      3.0,
      colors::LIGHTGRAY,
    );
    draw_line_ex(
      Vector2 {
        x: protractor_position.x,
        y: protractor_position.y - 90.0,
      },
      Vector2 {
        x: protractor_position.x,
        y: protractor_position.y + 90.0,
      },
      3.0,
      colors::LIGHTGRAY,
    );
    draw_line_ex(
      Vector2 {
        x: protractor_position.x - 80.0,
        y: protractor_position.y - 45.0,
      },
      Vector2 {
        x: protractor_position.x + 80.0,
        y: protractor_position.y + 45.0,
      },
      3.0,
      colors::GREEN,
    );
    draw_line_ex(
      Vector2 {
        x: protractor_position.x - 80.0,
        y: protractor_position.y + 45.0,
      },
      Vector2 {
        x: protractor_position.x + 80.0,
        y: protractor_position.y - 45.0,
      },
      3.0,
      colors::GREEN,
    );
    draw_text(
      "0",
      protractor_position.x as i32 + 96,
      protractor_position.y as i32 - 9,
      20,
      colors::BLACK,
    );
    draw_text(
      "30",
      protractor_position.x as i32 + 74,
      protractor_position.y as i32 - 68,
      20,
      colors::BLACK,
    );
    draw_text(
      "90",
      protractor_position.x as i32 - 11,
      protractor_position.y as i32 - 110,
      20,
      colors::BLACK,
    );
    draw_text(
      "150",
      protractor_position.x as i32 - 100,
      protractor_position.y as i32 - 68,
      20,
      colors::BLACK,
    );
    draw_text(
      "180",
      protractor_position.x as i32 - 124,
      protractor_position.y as i32 - 9,
      20,
      colors::BLACK,
    );
    draw_text(
      "210",
      protractor_position.x as i32 - 100,
      protractor_position.y as i32 + 50,
      20,
      colors::BLACK,
    );
    draw_text(
      "270",
      protractor_position.x as i32 - 18,
      protractor_position.y as i32 + 92,
      20,
      colors::BLACK,
    );
    draw_text(
      "330",
      protractor_position.x as i32 + 72,
      protractor_position.y as i32 + 50,
      20,
      colors::BLACK,
    );

    if current_angle_degrees != 0.0 {
      draw_line_ex(protractor_position, final_vector, 3.0, gesture_color);
    }

    if current_gesture != Gesture::None {
      if touch_count != 0 {
        for i in 0..touch_count {
          draw_circle_v(touch_position[i as usize], 50.0, fade(gesture_color, 0.5));
          draw_circle_v(touch_position[i as usize], 5.0, gesture_color);
        }

        if touch_count == 2 {
          draw_line_ex(
            touch_position[0],
            touch_position[1],
            if current_gesture == Gesture::PinchOut {
              8.0
            } else {
              12.0
            },
            gesture_color,
          );
        }
      } else {
        draw_circle_v(mouse_position, 35.0, fade(gesture_color, 0.5));
        draw_circle_v(mouse_position, 5.0, gesture_color);
      }
    }

    end_drawing();
  }

  close_window();
}

fn get_gesture_name(gesture: Gesture) -> &'static str {
  return match gesture {
    Gesture::None => "None",
    Gesture::Tap => "Tap",
    Gesture::DoubleTap => "Double Tap",
    Gesture::Hold => "Hold",
    Gesture::Drag => "Drag",
    Gesture::SwipeRight => "Swipe Right",
    Gesture::SwipeLeft => "Swipe Left",
    Gesture::SwipeUp => "Swipe Up",
    Gesture::SwipeDown => "Swipe Down",
    Gesture::PinchIn => "Pinch In",
    Gesture::PinchOut => "Pinch Out",
  };
}

fn get_gesture_color(gesture: Gesture) -> Color {
  return match gesture {
    Gesture::None => colors::BLACK,
    Gesture::Tap => colors::BLUE,
    Gesture::DoubleTap => colors::SKYBLUE,
    Gesture::Hold => colors::BLACK,
    Gesture::Drag => colors::LIME,
    Gesture::SwipeRight => colors::RED,
    Gesture::SwipeLeft => colors::RED,
    Gesture::SwipeUp => colors::RED,
    Gesture::SwipeDown => colors::RED,
    Gesture::PinchIn => colors::VIOLET,
    Gesture::PinchOut => colors::ORANGE,
  };
}
