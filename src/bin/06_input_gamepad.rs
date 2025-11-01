use std::mem::transmute;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing,
    gamepad::{
      get_gamepad_axis_count, get_gamepad_axis_movement, get_gamepad_button_pressed,
      get_gamepad_name, is_gamepad_available, is_gamepad_button_down,
    },
    init_window,
    keyboard::is_key_pressed,
    set_config_flags, set_target_fps, window_should_close,
  },
  enums::{ConfigFlags, GamepadAxis, GamepadButton, KeyboardKey},
  shape::{draw_circle, draw_rectangle, draw_rectangle_rounded, draw_triangle},
  structs::{Rectangle, Vector2},
  text::draw_text,
  texture::{draw_texture, load_texture, unload_texture},
};

fn main() {
  const XBOX_ALIAS_1: &str = "xbox";
  const XBOX_ALIAS_2: &str = "x-box";
  const PS_ALIAS: &str = "playstation";

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(ConfigFlags::MSAA4xHint);

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

  let mut gamepad = 0;

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

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
        colors::BLACK,
      );

      let mut left_stick_x = get_gamepad_axis_movement(gamepad, GamepadAxis::LeftX);
      let mut left_stick_y = get_gamepad_axis_movement(gamepad, GamepadAxis::LeftY);
      let mut right_stick_x = get_gamepad_axis_movement(gamepad, GamepadAxis::RightX);
      let mut right_stick_y = get_gamepad_axis_movement(gamepad, GamepadAxis::RightY);
      let mut left_trigger = get_gamepad_axis_movement(gamepad, GamepadAxis::LeftTrigger);
      let mut right_trigger = get_gamepad_axis_movement(gamepad, GamepadAxis::RightTrigger);

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

      if get_gamepad_name(gamepad)
        .to_lowercase()
        .contains(XBOX_ALIAS_1)
        || get_gamepad_name(gamepad)
          .to_lowercase()
          .contains(XBOX_ALIAS_2)
      {
        draw_texture(tex_xbox_pad, 0, 0, colors::DARKGRAY);

        if is_gamepad_button_down(gamepad, GamepadButton::Middle) {
          draw_circle(394, 89, 19.0, colors::RED);
        }

        if is_gamepad_button_down(gamepad, GamepadButton::MiddleRight) {
          draw_circle(436, 150, 9.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::MiddleLeft) {
          draw_circle(352, 150, 9.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceLeft) {
          draw_circle(501, 151, 15.0, colors::BLUE);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceDown) {
          draw_circle(536, 187, 15.0, colors::LIME);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceRight) {
          draw_circle(572, 151, 15.0, colors::MAROON);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceUp) {
          draw_circle(536, 115, 15.0, colors::GOLD);
        }

        draw_rectangle(317, 202, 19, 71, colors::BLACK);
        draw_rectangle(293, 228, 69, 19, colors::BLACK);
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceUp) {
          draw_rectangle(317, 202, 19, 26, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceDown) {
          draw_rectangle(317, 202 + 45, 19, 26, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceLeft) {
          draw_rectangle(292, 228, 25, 19, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceRight) {
          draw_rectangle(292 + 44, 228, 26, 19, colors::RED);
        }

        if is_gamepad_button_down(gamepad, GamepadButton::LeftTrigger1) {
          draw_circle(259, 61, 20.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightTrigger1) {
          draw_circle(536, 61, 20.0, colors::RED);
        }

        let mut left_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::LeftThumb) {
          left_gamepad_color = colors::RED;
        }
        draw_circle(259, 152, 39.0, colors::BLACK);
        draw_circle(259, 152, 34.0, colors::LIGHTGRAY);
        draw_circle(
          259 + (left_stick_x * 20.0) as i32,
          152 + (left_stick_y * 20.0) as i32,
          25.0,
          left_gamepad_color,
        );

        let mut right_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::RightThumb) {
          right_gamepad_color = colors::RED;
        }
        draw_circle(461, 237, 38.0, colors::BLACK);
        draw_circle(461, 237, 33.0, colors::LIGHTGRAY);
        draw_circle(
          461 + (right_stick_x * 20.0) as i32,
          237 + (right_stick_y * 20.0) as i32,
          25.0,
          right_gamepad_color,
        );

        draw_rectangle(170, 30, 15, 70, colors::GRAY);
        draw_rectangle(604, 30, 15, 70, colors::GRAY);
        draw_rectangle(
          170,
          30,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
        draw_rectangle(
          604,
          30,
          15,
          (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
      } else if get_gamepad_name(gamepad).to_lowercase().contains(PS_ALIAS) {
        draw_texture(tex_ps3_pad, 0, 0, colors::DARKGRAY);

        if is_gamepad_button_down(gamepad, GamepadButton::Middle) {
          draw_circle(396, 222, 13.0, colors::RED);
        }

        if is_gamepad_button_down(gamepad, GamepadButton::MiddleLeft) {
          draw_rectangle(328, 170, 32, 13, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::MiddleRight) {
          draw_triangle(
            Vector2 { x: 436.0, y: 168.0 },
            Vector2 { x: 436.0, y: 185.0 },
            Vector2 { x: 464.0, y: 177.0 },
            colors::RED,
          );
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceUp) {
          draw_circle(557, 144, 13.0, colors::LIME);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceRight) {
          draw_circle(586, 173, 13.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceDown) {
          draw_circle(557, 203, 13.0, colors::VIOLET);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceLeft) {
          draw_circle(527, 173, 13.0, colors::PINK);
        }

        draw_rectangle(225, 132, 24, 84, colors::BLACK);
        draw_rectangle(195, 161, 84, 25, colors::BLACK);
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceUp) {
          draw_rectangle(225, 132, 24, 29, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceDown) {
          draw_rectangle(225, 132 + 54, 24, 30, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceLeft) {
          draw_rectangle(195, 161, 30, 25, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceRight) {
          draw_rectangle(195 + 54, 161, 30, 25, colors::RED);
        }

        if is_gamepad_button_down(gamepad, GamepadButton::LeftTrigger1) {
          draw_circle(239, 82, 20.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightTrigger1) {
          draw_circle(557, 82, 20.0, colors::RED);
        }

        let mut left_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::LeftThumb) {
          left_gamepad_color = colors::RED;
        }
        draw_circle(319, 255, 35.0, colors::BLACK);
        draw_circle(319, 255, 31.0, colors::LIGHTGRAY);
        draw_circle(
          319 + (left_stick_x * 20.0) as i32,
          255 + (left_stick_y * 20.0) as i32,
          25.0,
          left_gamepad_color,
        );

        let mut right_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::RightThumb) {
          right_gamepad_color = colors::RED;
        }
        draw_circle(475, 255, 35.0, colors::BLACK);
        draw_circle(475, 255, 31.0, colors::LIGHTGRAY);
        draw_circle(
          475 + (right_stick_x * 20.0) as i32,
          255 + (right_stick_y * 20.0) as i32,
          25.0,
          right_gamepad_color,
        );

        draw_rectangle(169, 48, 15, 70, colors::GRAY);
        draw_rectangle(611, 48, 15, 70, colors::GRAY);
        draw_rectangle(
          169,
          48,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
        draw_rectangle(
          611,
          48,
          15,
          (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
      } else {
        draw_rectangle_rounded(
          Rectangle {
            x: 175.0,
            y: 110.0,
            width: 460.0,
            height: 220.0,
          },
          0.3,
          16,
          colors::DARKGRAY,
        );

        draw_circle(365, 170, 12.0, colors::RAYWHITE);
        draw_circle(405, 170, 12.0, colors::RAYWHITE);
        draw_circle(445, 170, 12.0, colors::RAYWHITE);
        draw_circle(516, 191, 17.0, colors::RAYWHITE);
        draw_circle(551, 227, 17.0, colors::RAYWHITE);
        draw_circle(587, 191, 17.0, colors::RAYWHITE);
        draw_circle(551, 155, 17.0, colors::RAYWHITE);
        if is_gamepad_button_down(gamepad, GamepadButton::MiddleLeft) {
          draw_circle(365, 170, 10.0, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::Middle) {
          draw_circle(405, 170, 10.0, colors::GREEN);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::MiddleRight) {
          draw_circle(445, 170, 10.0, colors::BLUE);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceLeft) {
          draw_circle(516, 191, 15.0, colors::GOLD);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceDown) {
          draw_circle(551, 227, 15.0, colors::BLUE);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceRight) {
          draw_circle(587, 191, 15.0, colors::GREEN);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightFaceUp) {
          draw_circle(551, 155, 15.0, colors::RED);
        }

        draw_rectangle(245, 145, 28, 88, colors::RAYWHITE);
        draw_rectangle(215, 174, 88, 29, colors::RAYWHITE);
        draw_rectangle(247, 147, 24, 84, colors::BLACK);
        draw_rectangle(217, 176, 84, 25, colors::BLACK);
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceUp) {
          draw_rectangle(247, 147, 24, 29, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceDown) {
          draw_rectangle(247, 147 + 54, 24, 30, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceLeft) {
          draw_rectangle(217, 176, 30, 25, colors::RED);
        }
        if is_gamepad_button_down(gamepad, GamepadButton::LeftFaceRight) {
          draw_rectangle(217 + 54, 176, 30, 25, colors::RED);
        }

        draw_rectangle_rounded(
          Rectangle {
            x: 215.0,
            y: 98.0,
            width: 100.0,
            height: 10.0,
          },
          0.5,
          16,
          colors::DARKGRAY,
        );
        draw_rectangle_rounded(
          Rectangle {
            x: 495.0,
            y: 98.0,
            width: 100.0,
            height: 10.0,
          },
          0.5,
          16,
          colors::DARKGRAY,
        );
        if is_gamepad_button_down(gamepad, GamepadButton::LeftTrigger1) {
          draw_rectangle_rounded(
            Rectangle {
              x: 215.0,
              y: 98.0,
              width: 100.0,
              height: 10.0,
            },
            0.5,
            16,
            colors::RED,
          );
        }
        if is_gamepad_button_down(gamepad, GamepadButton::RightTrigger1) {
          draw_rectangle_rounded(
            Rectangle {
              x: 495.0,
              y: 98.0,
              width: 100.0,
              height: 10.0,
            },
            0.5,
            16,
            colors::RED,
          );
        }

        let mut left_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::LeftThumb) {
          left_gamepad_color = colors::RED;
        }
        draw_circle(345, 260, 40.0, colors::BLACK);
        draw_circle(345, 260, 35.0, colors::LIGHTGRAY);
        draw_circle(
          345 + (left_stick_x * 20.0) as i32,
          260 + (left_stick_y * 20.0) as i32,
          25.0,
          left_gamepad_color,
        );

        let mut right_gamepad_color = colors::BLACK;
        if is_gamepad_button_down(gamepad, GamepadButton::RightThumb) {
          right_gamepad_color = colors::RED;
        }
        draw_circle(465, 260, 40.0, colors::BLACK);
        draw_circle(465, 260, 35.0, colors::LIGHTGRAY);
        draw_circle(
          465 + (right_stick_x * 20.0) as i32,
          260 + (right_stick_y * 20.0) as i32,
          25.0,
          right_gamepad_color,
        );

        draw_rectangle(151, 110, 15, 70, colors::GRAY);
        draw_rectangle(644, 110, 15, 70, colors::GRAY);
        draw_rectangle(
          151,
          110,
          15,
          (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
        draw_rectangle(
          644,
          110,
          15,
          (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
          colors::RED,
        );
      }

      draw_text(
        &format!("DETECTED AXIS [{}]:", get_gamepad_axis_count(gamepad)),
        10,
        50,
        10,
        colors::MAROON,
      );

      for i in 0..get_gamepad_axis_count(gamepad) {
        draw_text(
          &format!(
            "AXIS {}: {}",
            i,
            get_gamepad_axis_movement(gamepad, unsafe { transmute(i as i8) })
          ),
          20,
          70 + 20 * i,
          10,
          colors::DARKGRAY,
        );
      }

      if get_gamepad_button_pressed() != GamepadButton::Unknown {
        draw_text(
          &format!("DETECTED BUTTON: {}", get_gamepad_button_pressed() as i32),
          10,
          430,
          10,
          colors::RED,
        );
      } else {
        draw_text("DETECTED BUTTON: NONE", 10, 430, 10, colors::GRAY);
      }
    } else {
      draw_text(
        &format!("GP{}: NOT DETECTED", gamepad),
        10,
        10,
        10,
        colors::GRAY,
      );

      draw_texture(tex_xbox_pad, 0, 0, colors::LIGHTGRAY);
    }

    end_drawing();
  }

  unload_texture(tex_ps3_pad);
  unload_texture(tex_xbox_pad);

  close_window();
}
