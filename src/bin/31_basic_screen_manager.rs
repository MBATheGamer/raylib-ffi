enum GameScreen {
  Logo = 0,
  Title,
  Gameplay,
  Ending,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - basic screen manager",
  );

  let current_screen = GameScreen::Logo;

  let frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    match current_screen {
      GameScreen::Logo => {
        frames_counter += 1;

        if frames_counter > 120 {
          current_screen = GameScreen::Title;
        }
      }
      GameScreen::Title => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::Tap) {
          current_screen = GameScreen::Gameplay;
        }
      }
      GameScreen::Gameplay => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::Tap) {
          current_screen = GameScreen::Ending;
        }
      }
      GameScreen::Ending => {
        if is_key_pressed(KeyboardKey::KeyEnter) || is_gesture_detected(Gesture::Tap) {
          current_screen = GameScreen::Title;
        }
      }
      _ => {}
    }
    begin_drawing();

    clear_background(colors::RAYWHITE);

    match current_screen {
      GameScreen::Logo => {
        draw_text("LOGO SCREEN", 20, 20, 40, colors::LIGHTGRAY);
        draw_text("WAIT for 2 SECONDS...", 290, 220, 20, colors::GRAY);
      }
      GameScreen::Title => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, colors::GREEN);
        draw_text("TITLE SCREEN", 20, 20, 40, colors::DARKGREEN);
        draw_text(
          "PRESS ENTER or TAP to JUMP to GAMEPLAY SCREEN",
          120,
          220,
          20,
          colors::DARKGREEN,
        );
      }
      GameScreen::Gameplay => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, colors::PURPLE);
        draw_text("GAMEPLAY SCREEN", 20, 20, 40, colors::MAROON);
        draw_text(
          "PRESS ENTER or TAP to JUMP to ENDING SCREEN",
          130,
          220,
          20,
          colors::MAROON,
        );
      }
      GameScreen::Ending => {
        draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, colors::BLUE);
        draw_text("ENDING SCREEN", 20, 20, 40, colors::DARKBLUE);
        draw_text(
          "PRESS ENTER or TAP to RETURN to TITLE SCREEN",
          120,
          220,
          20,
          colors::DARKBLUE,
        );
      }
      _ => {}
    }

    end_drawing();
  }

  close_window();
}
