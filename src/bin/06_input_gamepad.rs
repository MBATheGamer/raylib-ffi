fn main() {
  const XBOX_ALIAS_1: &str = "xbox";
  const XBOX_ALIAS_2: &str = "x-box";
  const PS_ALIAS: &str = "playstation";

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(FLAG_MSAA_4X_HINT);

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input gamepad",
  );

  let tex_ps3_pad = load_texture("resources/ps3.png");
  let tex_xbox_pad = load_texture("resources/xbox.png");

  const LEFT_STICK_DEADZONE_X: f32 = 0.1;
  const LEFT_STICK_DEADZONE_Y: f32 = 0.1;
  const RIGHT_STICK_DEADZONE_X: f32 = 0.1;
  const RIGHT_STICK_DEADZONE_Y: f32 = 0.1;
  const LEFT_TRIGGER_DEADZONE: f32 = -0.9;
  const RIGHT_TRIGGER_DEADZONE: f32 = -0.9;

  set_target_fps(60);

  let gamepad = 0;

  while !window_should_close() {
    begin_drawing();

    clear_background(RAYWHITE);

    if is_key_pressed(KeyboardKey::KeyLeft) && gamepad > 0 {
      gamepad -= 1;
    }
    if is_key_pressed(KeyboardKey::KeyRight) {
      gamepad += 1;
    }

    if is_gamepad_available(gamepad) {
      draw_text(
        &format!("GP{}: {}", gamepad, get_gamepad_name(gamepad)),
        10,
        10,
        10,
        BLACK,
      );

      let left_stick_x = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_LEFT_X);
      let left_stick_y = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_LEFT_Y);
      let right_stick_x = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_RIGHT_X);
      let right_stick_y = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_RIGHT_Y);
      let left_trigger = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_LEFT_TRIGGER);
      let right_trigger = get_gamepad_axis_movement(gamepad, GAMEPAD_AXIS_RIGHT_TRIGGER);

      if left_stick_x > -LEFT_STICK_DEADZONE_X && left_stick_x < LEFT_STICK_DEADZONE_X {
        left_stick_x = 0.0;
      }
      if left_stick_y > -LEFT_STICK_DEADZONE_Y && left_stick_y < LEFT_STICK_DEADZONE_Y {
        left_stick_y = 0.0;
      }
      if right_stick_x > -RIGHT_STICK_DEADZONE_X && right_stick_x < RIGHT_STICK_DEADZONE_X {
        right_stick_x = 0.0;
      }
      if right_stick_y > -RIGHT_STICK_DEADZONE_Y && right_stick_y < RIGHT_STICK_DEADZONE_Y {
        right_stick_y = 0.0;
      }
      if left_trigger < LEFT_TRIGGER_DEADZONE {
        left_trigger = -1.0;
      }
      if right_trigger < RIGHT_TRIGGER_DEADZONE {
        right_trigger = -1.0;
      }

      if text_find_index(text_to_lower(get_gamepad_name(gamepad)), XBOX_ALIAS_1) > -1
        || text_find_index(text_to_lower(get_gamepad_name(gamepad)), XBOX_ALIAS_2) > -1
      {
        draw_texture(tex_xbox_pad, 0, 0, DARKGRAY);

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE) {
          draw_circle(394, 89, 19, RED);
        }

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
          draw_circle(436, 150, 9, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
          draw_circle(352, 150, 9, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
          draw_circle(501, 151, 15, BLUE);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
          draw_circle(536, 187, 15, LIME);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
          draw_circle(572, 151, 15, MAROON);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
          draw_circle(536, 115, 15, GOLD);
        }

        draw_rectangle(317, 202, 19, 71, BLACK);
        draw_rectangle(293, 228, 69, 19, BLACK);
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
          draw_rectangle(317, 202, 19, 26, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
          draw_rectangle(317, 202 + 45, 19, 26, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
          draw_rectangle(292, 228, 25, 19, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
          draw_rectangle(292 + 44, 228, 26, 19, RED);
        }

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
          draw_circle(259, 61, 20, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
          draw_circle(536, 61, 20, RED);
        }

        let left_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
          left_gamepad_color = RED;
        }
        draw_circle(259, 152, 39, BLACK);
        draw_circle(259, 152, 34, LIGHTGRAY);
        draw_circle(
          259 + (left_stick_x * 20.0) as i32,
          152 + (left_stick_y * 20.0) as i32,
          25,
          left_gamepad_color,
        );

        let right_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
          right_gamepad_color = RED;
        }
        draw_circle(461, 237, 38, BLACK);
        draw_circle(461, 237, 33, LIGHTGRAY);
        draw_circle(
          461 + (right_stick_x * 20.0) as i32,
          237 + (right_stick_y * 20.0) as i32,
          25,
          right_gamepad_color,
        );

        draw_rectangle(170, 30, 15, 70, GRAY);
        draw_rectangle(604, 30, 15, 70, GRAY);
        draw_rectangle(
          170,
          30,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          RED,
        );
        draw_rectangle(
          604,
          30,
          15,
          (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
          RED,
        );
      } else if text_find_index(text_to_lower(get_gamepad_name(gamepad)), PS_ALIAS) > -1 {
        draw_texture(tex_ps3_pad, 0, 0, DARKGRAY);

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE) {
          draw_circle(396, 222, 13, RED);
        }

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
          draw_rectangle(328, 170, 32, 13, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
          draw_triangle(
            Vector2 { x: 436.0, y: 168.0 },
            Vector2 { x: 436.0, y: 185.0 },
            Vector2 { x: 464.0, y: 177.0 },
            RED,
          );
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
          draw_circle(557, 144, 13, LIME);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
          draw_circle(586, 173, 13, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
          draw_circle(557, 203, 13, VIOLET);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
          draw_circle(527, 173, 13, PINK);
        }

        draw_rectangle(225, 132, 24, 84, BLACK);
        draw_rectangle(195, 161, 84, 25, BLACK);
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
          draw_rectangle(225, 132, 24, 29, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
          draw_rectangle(225, 132 + 54, 24, 30, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
          draw_rectangle(195, 161, 30, 25, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
          draw_rectangle(195 + 54, 161, 30, 25, RED);
        }

        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
          draw_circle(239, 82, 20, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
          draw_circle(557, 82, 20, RED);
        }

        let left_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
          left_gamepad_color = RED;
        }
        draw_circle(319, 255, 35, BLACK);
        draw_circle(319, 255, 31, LIGHTGRAY);
        draw_circle(
          319 + (left_stick_x * 20.0) as i32,
          255 + (left_stick_y * 20.0) as i32,
          25,
          left_gamepad_color,
        );

        let right_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
          right_gamepad_color = RED;
        }
        draw_circle(475, 255, 35, BLACK);
        draw_circle(475, 255, 31, LIGHTGRAY);
        draw_circle(
          475 + (right_stick_x * 20.0) as i32,
          255 + (right_stick_y * 20.0) as i32,
          25,
          right_gamepad_color,
        );

        draw_rectangle(169, 48, 15, 70, GRAY);
        draw_rectangle(611, 48, 15, 70, GRAY);
        draw_rectangle(
          169,
          48,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          RED,
        );
        draw_rectangle(
          611,
          48,
          15,
          (((1.0 + right_trigger) / 2.0) * 70) as i32,
          RED,
        );
      } else {
        draw_rectangle_rounded(
          Rectangle {
            x: 175,
            y: 110,
            width: 460,
            height: 220,
          },
          0.3,
          16,
          DARKGRAY,
        );

        draw_circle(365, 170, 12, RAYWHITE);
        draw_circle(405, 170, 12, RAYWHITE);
        draw_circle(445, 170, 12, RAYWHITE);
        draw_circle(516, 191, 17, RAYWHITE);
        draw_circle(551, 227, 17, RAYWHITE);
        draw_circle(587, 191, 17, RAYWHITE);
        draw_circle(551, 155, 17, RAYWHITE);
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
          draw_circle(365, 170, 10, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE) {
          draw_circle(405, 170, 10, GREEN);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
          draw_circle(445, 170, 10, BLUE);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
          draw_circle(516, 191, 15, GOLD);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
          draw_circle(551, 227, 15, BLUE);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
          draw_circle(587, 191, 15, GREEN);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
          draw_circle(551, 155, 15, RED);
        }

        draw_rectangle(245, 145, 28, 88, RAYWHITE);
        draw_rectangle(215, 174, 88, 29, RAYWHITE);
        draw_rectangle(247, 147, 24, 84, BLACK);
        draw_rectangle(217, 176, 84, 25, BLACK);
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
          draw_rectangle(247, 147, 24, 29, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
          draw_rectangle(247, 147 + 54, 24, 30, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
          draw_rectangle(217, 176, 30, 25, RED);
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
          draw_rectangle(217 + 54, 176, 30, 25, RED);
        }

        draw_rectangle_rounded(
          Rectangle {
            x: 215,
            y: 98,
            width: 100,
            height: 10,
          },
          0.5,
          16,
          DARKGRAY,
        );
        draw_rectangle_rounded(
          Rectangle {
            x: 495,
            y: 98,
            width: 100,
            height: 10,
          },
          0.5,
          16,
          DARKGRAY,
        );
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
          draw_rectangle_rounded(
            Rectangle {
              x: 215,
              y: 98,
              width: 100,
              height: 10,
            },
            0.5,
            16,
            RED,
          );
        }
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
          draw_rectangle_rounded(
            Rectangle {
              x: 495,
              y: 98,
              width: 100,
              height: 10,
            },
            0.5,
            16,
            RED,
          );
        }

        let left_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
          left_gamepad_color = RED;
        }
        draw_circle(345, 260, 40, BLACK);
        draw_circle(345, 260, 35, LIGHTGRAY);
        draw_circle(
          345 + (left_stick_x * 20.0) as i32,
          260 + (left_stick_y * 20.0) as i32,
          25,
          left_gamepad_color,
        );

        let right_gamepad_color = BLACK;
        if is_gamepad_button_down(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
          right_gamepad_color = RED;
        }
        draw_circle(465, 260, 40, BLACK);
        draw_circle(465, 260, 35, LIGHTGRAY);
        draw_circle(
          465 + (right_stick_x * 20.0) as i32,
          260 + (right_stick_y * 20.0) as i32,
          25,
          right_gamepad_color,
        );

        draw_rectangle(151, 110, 15, 70, GRAY);
        draw_rectangle(644, 110, 15, 70, GRAY);
        draw_rectangle(
          151,
          110,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          RED,
        );
        draw_rectangle(
          644,
          110,
          15,
          (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
          RED,
        );
      }

      draw_text(
        &format!("DETECTED AXIS [{}]:", get_gamepad_axis_count(gamepad)),
        10,
        50,
        10,
        MAROON,
      );

      for i in 0..get_gamepad_axis_count(gamepad) {
        draw_text(
          &format!("AXIS {}: {}", i, get_gamepad_axis_movement(gamepad, i)),
          20,
          70 + 20 * i,
          10,
          DARKGRAY,
        );
      }

      if get_gamepad_button_pressed() != GAMEPAD_BUTTON_UNKNOWN {
        draw_text(
          &format!("DETECTED BUTTON: {}", get_gamepad_button_pressed()),
          10,
          430,
          10,
          RED,
        );
      } else {
        draw_text("DETECTED BUTTON: NONE", 10, 430, 10, GRAY);
      }
    } else {
      draw_text(&format!("GP{}: NOT DETECTED", gamepad), 10, 10, 10, GRAY);

      draw_texture(tex_xbox_pad, 0, 0, LIGHTGRAY);
    }

    end_drawing();
  }

  unload_texture(tex_ps3_pad);
  unload_texture(tex_xbox_pad);

  close_window();
}
