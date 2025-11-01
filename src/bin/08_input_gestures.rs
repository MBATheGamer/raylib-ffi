use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, gesture::get_gesture_detected,
    init_window, set_target_fps, touch::get_touch_position, window_should_close,
  },
  enums::Gesture,
  shape::{
    check_collision_point_rec, draw_circle_v, draw_rectangle, draw_rectangle_lines,
    draw_rectangle_rec,
  },
  structs::Rectangle,
  text::draw_text,
  texture::fade,
};

fn main() {
  const MAX_GESTURE_STRINGS: usize = 20;
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input gestures",
  );

  let touch_area: Rectangle = Rectangle {
    x: 220.0,
    y: 10.0,
    width: SCREEN_WIDTH as f32 - 230.0,
    height: SCREEN_HEIGHT as f32 - 20.0,
  };

  let mut gestures_count = 0;
  let mut gesture_strings: Vec<&str> = vec![];

  let mut current_gesture = Gesture::None;

  set_target_fps(60);

  while !window_should_close() {
    let last_gesture = current_gesture;
    current_gesture = get_gesture_detected();
    let touch_position = get_touch_position(0);

    if check_collision_point_rec(touch_position, touch_area) && (current_gesture != Gesture::None) {
      if current_gesture != last_gesture {
        match current_gesture {
          Gesture::Tap => gesture_strings.push("GESTURE TAP"),
          Gesture::DoubleTap => gesture_strings.push("GESTURE DOUBLETAP"),
          Gesture::Hold => gesture_strings.push("GESTURE HOLD"),
          Gesture::Drag => gesture_strings.push("GESTURE DRAG"),
          Gesture::SwipeRight => gesture_strings.push("GESTURE SWIPE RIGHT"),
          Gesture::SwipeLeft => gesture_strings.push("GESTURE SWIPE LEFT"),
          Gesture::SwipeUp => gesture_strings.push("GESTURE SWIPE UP"),
          Gesture::SwipeDown => gesture_strings.push("GESTURE SWIPE DOWN"),
          Gesture::PinchIn => gesture_strings.push("GESTURE PINCH IN"),
          Gesture::PinchOut => gesture_strings.push("GESTURE PINCH OUT"),
          _ => {}
        };

        gestures_count += 1;

        if gestures_count >= MAX_GESTURE_STRINGS {
          gesture_strings = vec![];

          gestures_count = 0;
        }
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_rectangle_rec(touch_area, colors::GRAY);
    draw_rectangle(
      225,
      15,
      SCREEN_WIDTH - 240,
      SCREEN_HEIGHT - 30,
      colors::RAYWHITE,
    );

    draw_text(
      "GESTURES TEST AREA",
      SCREEN_WIDTH - 270,
      SCREEN_HEIGHT - 40,
      20,
      fade(colors::GRAY, 0.5),
    );

    for i in 0..gestures_count {
      if i % 2 == 0 {
        draw_rectangle(
          10,
          30 + 20 * i as i32,
          200,
          20,
          fade(colors::LIGHTGRAY, 0.5),
        );
      } else {
        draw_rectangle(
          10,
          30 + 20 * i as i32,
          200,
          20,
          fade(colors::LIGHTGRAY, 0.3),
        );
      }
      if i < gestures_count - 1 {
        draw_text(
          &gesture_strings[i],
          35,
          36 + 20 * i as i32,
          10,
          colors::DARKGRAY,
        );
      } else {
        draw_text(
          &gesture_strings[i],
          35,
          36 + 20 * i as i32,
          10,
          colors::MAROON,
        );
      }
    }

    draw_rectangle_lines(10, 29, 200, SCREEN_HEIGHT - 50, colors::GRAY);
    draw_text("DETECTED GESTURES", 50, 15, 10, colors::GRAY);

    if current_gesture != Gesture::None {
      draw_circle_v(touch_position, 30.0, colors::MAROON);
    }
    end_drawing();
  }

  close_window();
}
