fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - logo raylib anim",
  );

  let logo_position_x = SCREEN_WIDTH / 2 - 128;
  let logo_position_y = SCREEN_HEIGHT / 2 - 128;

  let frames_counter = 0;
  let letters_count = 0;

  let top_side_rec_width = 16;
  let left_side_rec_height = 16;

  let bottom_side_rec_width = 16;
  let right_side_rec_height = 16;

  let state = 0;
  let alpha = 1.0;

  set_target_fps(60);

  while !window_should_close() {
    if state == 0 {
      frames_counter += 1;

      if frames_counter == 120 {
        state = 1;
        frames_counter = 0;
      }
    } else if state == 1 {
      top_side_rec_width += 4;
      left_side_rec_height += 4;

      if top_side_rec_width == 256 {
        state = 2;
      }
    } else if state == 2 {
      bottom_side_rec_width += 4;
      right_side_rec_height += 4;

      if bottom_side_rec_width == 256 {
        state = 3;
      }
    } else if state == 3 {
      frames_counter += 1;

      if frames_counter / 12 != 0 {
        letters_count += 1;
        frames_counter = 0;
      }

      if letters_count >= 10 {
        alpha -= 0.02;

        if alpha <= 0.0 {
          alpha = 0.0;
          state = 4;
        }
      }
    } else if state == 4 {
      if is_key_pressed(KeyboardKey::KeyR) {
        frames_counter = 0;
        letters_count = 0;

        top_side_rec_width = 16;
        left_side_rec_height = 16;

        bottom_side_rec_width = 16;
        right_side_rec_height = 16;

        alpha = 1.0;
        state = 0;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if state == 0 {
      if (frames_counter / 15) % 2 != 0 {
        draw_rectangle(logo_position_x, logo_position_y, 16, 16, colors::BLACK);
      }
    } else if state == 1 {
      draw_rectangle(
        logo_position_x,
        logo_position_y,
        top_side_rec_width,
        16,
        colors::BLACK,
      );
      draw_rectangle(
        logo_position_x,
        logo_position_y,
        16,
        left_side_rec_height,
        colors::BLACK,
      );
    } else if state == 2 {
      draw_rectangle(
        logo_position_x,
        logo_position_y,
        top_side_rec_width,
        16,
        colors::BLACK,
      );
      draw_rectangle(
        logo_position_x,
        logo_position_y,
        16,
        left_side_rec_height,
        colors::BLACK,
      );

      draw_rectangle(
        logo_position_x + 240,
        logo_position_y,
        16,
        right_side_rec_height,
        colors::BLACK,
      );
      draw_rectangle(
        logo_position_x,
        logo_position_y + 240,
        bottom_side_rec_width,
        16,
        colors::BLACK,
      );
    } else if state == 3 {
      draw_rectangle(
        logo_position_x,
        logo_position_y,
        top_side_rec_width,
        16,
        fade(colors::BLACK, alpha),
      );
      draw_rectangle(
        logo_position_x,
        logo_position_y + 16,
        16,
        left_side_rec_height - 32,
        fade(colors::BLACK, alpha),
      );

      draw_rectangle(
        logo_position_x + 240,
        logo_position_y + 16,
        16,
        right_side_rec_height - 32,
        fade(colors::BLACK, alpha),
      );
      draw_rectangle(
        logo_position_x,
        logo_position_y + 240,
        bottom_side_rec_width,
        16,
        fade(colors::BLACK, alpha),
      );

      draw_rectangle(
        get_screen_width() / 2 - 112,
        get_screen_height() / 2 - 112,
        224,
        224,
        fade(colors::RAYWHITE, alpha),
      );

      let text = match &"raylib".get(0..letters_count) {
        Some(text) => text,
        None => "raylib",
      };

      draw_text(
        text,
        get_screen_width() / 2 - 44,
        get_screen_height() / 2 + 48,
        50,
        fade(colors::BLACK, alpha),
      );
    } else if state == 4 {
      draw_text("[R] REPLAY", 340, 200, 20, colors::GRAY);
    }

    end_drawing();
  }

  close_window();
}
