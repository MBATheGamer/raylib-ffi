fn main() {
  const MAX_GESTURE_STRINGS: usize = 20;
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input gestures",
  );

  let touch_position: Vector2 = Vector2 { x: 0.0, y: 0.0 };
  let touch_area: Rectangle = Rectangle {
    x: 220.0,
    y: 10.0,
    width: SCREEN_WIDTH as f32 - 230.0,
    height: SCREEN_HEIGHT as f32 - 20.0,
  };

  let gestures_count = 0;
  let gesture_strings: Vec<&str> = vec![];

  let current_gesture = Gesture::None;
  let last_gesture = Gesture::None;

  set_target_fps(60);

  while !window_should_close() {
    last_gesture = current_gesture;
    current_gesture = get_gesture_detected();
    touch_position = get_touch_position(0);

    if check_collision_point_rec(touch_position, touch_area) && (current_gesture != Gesture::None) {
      if current_gesture != last_gesture {
        match current_gesture {
          Gesture::Tap => gesture_strings[gestures_count] = "GESTURE TAP",
          Gesture::DoubleTap => gesture_strings[gestures_count] = "GESTURE DOUBLETAP",
          Gesture::Hold => gesture_strings[gestures_count] = "GESTURE HOLD",
          Gesture::Drag => gesture_strings[gestures_count] = "GESTURE DRAG",
          Gesture::SwipeRight => gesture_strings[gestures_count] = "GESTURE SWIPE RIGHT",
          Gesture::SwipeLeft => gesture_strings[gestures_count] = "GESTURE SWIPE LEFT",
          Gesture::SwipeUp => gesture_strings[gestures_count] = "GESTURE SWIPE UP",
          Gesture::SwipeDown => gesture_strings[gestures_count] = "GESTURE SWIPE DOWN",
          Gesture::PinchIn => gesture_strings[gestures_count] = "GESTURE PINCH IN",
          Gesture::PinchOut => gesture_strings[gestures_count] = "GESTURE PINCH OUT",
          _ => {}
        }

        gestures_count += 1;

        if gestures_count >= MAX_GESTURE_STRINGS {
          for i in 0..MAX_GESTURE_STRINGS {
            gesture_strings = vec![];
          }

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
