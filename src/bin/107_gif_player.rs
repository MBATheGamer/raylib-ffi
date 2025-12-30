const MAX_FRAME_DELAY: usize = 20;
const MIN_FRAME_DELAY: usize = 1;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - gif player",
  );

  let anim_frames = 0;

  let im_scarfy_anim = load_image_anim("resources/scarfy_run.gif", &anim_frames);

  let tex_scarfy_anim = load_texture_from_image(im_scarfy_anim);

  let next_frame_data_offset = 0;

  let current_anim_frame = 0;
  let frame_delay = 8;
  let frame_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    frame_counter += 1;
    if frame_counter >= frame_delay {
      current_anim_frame += 1;
      if current_anim_frame >= anim_frames {
        current_anim_frame = 0;
      }

      next_frame_data_offset =
        im_scarfy_anim.width * im_scarfy_anim.height * 4 * current_anim_frame;

      update_texture(tex_scarfy_anim, unsafe {
        im_scarfy_anim.data.add(next_frame_data_offset as usize)
      } as *const Color);

      frame_counter = 0;
    }

    if is_key_pressed(KeyboardKey::KeyRight) {
      frame_delay += 1;
    } else if is_key_pressed(KeyboardKey::KeyLeft) {
      frame_delay -= 1;
    }

    if frame_delay > MAX_FRAME_DELAY {
      frame_delay = MAX_FRAME_DELAY;
    } else if frame_delay < MIN_FRAME_DELAY {
      frame_delay = MIN_FRAME_DELAY;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text(
      &format!("TOTAL GIF FRAMES:  {:02}", anim_frames),
      50,
      30,
      20,
      colors::LIGHTGRAY,
    );
    draw_text(
      &format!("CURRENT FRAME: {:02}", current_anim_frame),
      50,
      60,
      20,
      colors::GRAY,
    );
    draw_text(
      &format!(
        "CURRENT FRAME IMAGE.DATA OFFSET: {:02}",
        next_frame_data_offset
      ),
      50,
      90,
      20,
      colors::GRAY,
    );

    draw_text("FRAMES DELAY: ", 100, 305, 10, colors::DARKGRAY);
    draw_text(
      &format!("{:02} frames", frame_delay),
      620,
      305,
      10,
      colors::DARKGRAY,
    );
    draw_text(
      "PRESS RIGHT/LEFT KEYS to CHANGE SPEED!",
      290,
      350,
      10,
      colors::DARKGRAY,
    );

    for i in 0..MAX_FRAME_DELAY {
      if i < frame_delay {
        draw_rectangle((190 + 21 * i) as i32, 300, 20, 20, colors::RED);
      }
      {
        draw_rectangle_lines((190 + 21 * i) as i32, 300, 20, 20, colors::MAROON);
      }
    }

    draw_texture(
      tex_scarfy_anim,
      get_screen_width() / 2 - tex_scarfy_anim.width / 2,
      140,
      colors::WHITE,
    );

    draw_text(
      "(c) Scarfy sprite by Eiden Marsal",
      SCREEN_WIDTH - 200,
      SCREEN_HEIGHT - 20,
      10,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_texture(tex_scarfy_anim);
  unload_image(im_scarfy_anim);

  close_window();
}
