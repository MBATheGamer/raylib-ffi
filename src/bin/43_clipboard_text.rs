fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - clipboard text",
  );

  let clipboard_text: &'static str;

  let copyable_text: Vec<&'static str> = vec!["raylib is fun", "hello, clipboard!", "potato chips"];

  let text_index = 0;

  let popup_text: &'static str;

  const MAX_TIME: f32 = 3.0;
  let text_timer = 0.0;

  const ANIM_MAX_TIME: f32 = 0.1;
  let paste_anim = 0.0;
  let copy_anim = 0.0;
  let copy_anim_mult = 1;
  let text_anim = 0.0;
  let text_alpha = 0.0;

  const OFFSET_AMOUNT: i32 = -4;

  while !window_should_close() {
    let paste_pressed =
      is_key_down(KeyboardKey::KeyLeftControl) && is_key_pressed(KeyboardKey::KeyV);
    let copy_pressed =
      is_key_down(KeyboardKey::KeyLeftControl) && is_key_pressed(KeyboardKey::KeyC);

    if text_timer > 0.0 {
      text_timer -= get_frame_time();
    }
    if paste_anim > 0.0 {
      paste_anim -= get_frame_time();
    }
    if copy_anim > 0.0 {
      copy_anim -= get_frame_time();
    }
    if text_anim > 0.0 {
      text_anim -= get_frame_time();
    }

    if paste_pressed {
      let image = get_clipboard_image();

      if is_image_valid(image) {
        unload_image(image);
        popup_text = "clipboard contains image";
      } else {
        clipboard_text = get_clipboard_text();

        popup_text = "text pasted";
        paste_anim = ANIM_MAX_TIME;
      }

      text_timer = MAX_TIME;
      text_anim = ANIM_MAX_TIME;
      text_alpha = 1.0;
    }

    if copy_pressed {
      set_clipboard_text(copyable_text[text_index as usize]);

      text_timer = MAX_TIME;
      text_anim = ANIM_MAX_TIME;
      copy_anim = ANIM_MAX_TIME;
      copy_anim_mult = 1;
      text_alpha = 1.0;

      popup_text = "text copied";
    }

    if is_key_pressed(KeyboardKey::KeyUp) {
      copy_anim = ANIM_MAX_TIME;
      copy_anim_mult = 1;

      text_index += 1;

      if text_index >= copyable_text.len() as u32 {
        text_index = 0;
      }
    }

    if is_key_pressed(KeyboardKey::KeyDown) {
      copy_anim = ANIM_MAX_TIME;
      copy_anim_mult = -1;

      if text_index == 0 {
        text_index = copyable_text.len() as u32 - 1;
      } else {
        text_index -= 1;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if !clipboard_text.is_empty() {
      let offset = 0;
      if paste_anim > 0.0 {
        offset = OFFSET_AMOUNT;
      }

      draw_text("pasted clipboard:", 10, 10 + offset, 20, colors::DARKGREEN);
      draw_text(clipboard_text, 10, 30 + offset, 20, colors::DARKGRAY);
    }

    let text_offset = 0;
    if copy_anim > 0.0 {
      text_offset = OFFSET_AMOUNT;
    }

    draw_text(
      copyable_text[text_index as usize],
      10,
      330 + (text_offset * copy_anim_mult),
      20,
      colors::MAROON,
    );
    draw_text(
      "up/down to change string, ctrl-c to copy, ctrl-v to paste",
      10,
      355,
      20,
      colors::DARKGRAY,
    );

    if text_alpha > 0.0 {
      let offset = 0;
      if text_anim > 0.0 {
        offset = OFFSET_AMOUNT;
      }

      draw_text(
        popup_text,
        10,
        425 + offset,
        20,
        color_alpha(colors::DARKGREEN, text_alpha),
      );

      if text_timer < 0.0 {
        text_alpha -= get_frame_time();
      }
    }

    end_drawing();
  }

  close_window();
}
