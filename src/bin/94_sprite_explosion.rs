const NUM_FRAMES_PER_LINE: i32 = 5;
const NUM_LINES: i32 = 5;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - sprite explosion",
  );

  init_audio_device();

  let fx_boom = load_sound("resources/boom.wav");

  let explosion = load_texture("resources/explosion.png");

  let frame_width = (explosion.width / NUM_FRAMES_PER_LINE) as f32;
  let frame_height = (explosion.height / NUM_LINES) as f32;
  let current_frame = 0;
  let current_line = 0;

  let frame_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: frame_width,
    height: frame_height,
  };
  let position = Vector2 { x: 0.0, y: 0.0 };

  let active = false;
  let frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_pressed(MouseButton::Left) && !active {
      position = get_mouse_position();
      active = true;

      position.x -= frame_width / 2.0;
      position.y -= frame_height / 2.0;

      play_sound(fx_boom);
    }

    if active {
      frames_counter += 1;

      if frames_counter > 2 {
        current_frame += 1;

        if current_frame >= NUM_FRAMES_PER_LINE {
          current_frame = 0;
          current_line += 1;

          if current_line >= NUM_LINES {
            current_line = 0;
            active = false;
          }
        }

        frames_counter = 0;
      }
    }

    frame_rec.x = frame_width * current_frame as f32;
    frame_rec.y = frame_height * current_line as f32;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    if active {
      draw_texture_rec(explosion, frame_rec, position, colors::WHITE);
    }

    end_drawing();
  }

  unload_texture(explosion);
  unload_sound(fx_boom);

  close_audio_device();

  close_window();
}
