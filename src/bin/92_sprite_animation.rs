const MAX_FRAME_SPEED: i32 = 15;
const MIN_FRAME_SPEED: i32 = 1;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - sprite animation",
  );

  let scarfy = load_texture("resources/scarfy.png");

  let position = Vector2 { x: 350.0, y: 280.0 };
  let frame_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: scarfy.width as f32 / 6.0,
    height: scarfy.height as f32,
  };
  let current_frame = 0;

  let frames_counter = 0;
  let frames_speed = 8;

  set_target_fps(60);

  while !window_should_close() {
    frames_counter += 1;

    if frames_counter >= (60 / frames_speed) {
      frames_counter = 0;
      current_frame += 1;

      if current_frame > 5 {
        current_frame = 0;
      }

      frame_rec.x = current_frame as f32 * scarfy.width as f32 / 6.0;
    }

    if is_key_pressed(KeyboardKey::KeyRight) {
      frames_speed += 1;
    } else if is_key_pressed(KeyboardKey::KeyLeft) {
      frames_speed -= 1;
    }

    if frames_speed > MAX_FRAME_SPEED {
      frames_speed = MAX_FRAME_SPEED;
    } else if frames_speed < MIN_FRAME_SPEED {
      frames_speed = MIN_FRAME_SPEED;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture(scarfy, 15, 40, colors::WHITE);
    draw_rectangle_lines(15, 40, scarfy.width, scarfy.height, colors::LIME);
    draw_rectangle_lines(
      15 + frame_rec.x as i32,
      40 + frame_rec.y as i32,
      frame_rec.width as i32,
      frame_rec.height as i32,
      colors::RED,
    );

    draw_text("FRAME SPEED: ", 165, 210, 10, colors::DARKGRAY);
    draw_text(
      &format!("{} FPS", frames_speed),
      575,
      210,
      10,
      colors::DARKGRAY,
    );
    draw_text(
      "PRESS RIGHT/LEFT KEYS to CHANGE SPEED!",
      290,
      240,
      10,
      colors::DARKGRAY,
    );

    for i in 0..MAX_FRAME_SPEED {
      if i < frames_speed {
        draw_rectangle(250 + 21 * i, 205, 20, 20, colors::RED);
      }
      draw_rectangle_lines(250 + 21 * i, 205, 20, 20, colors::MAROON);
    }

    draw_texture_rec(scarfy, frame_rec, position, colors::WHITE);

    draw_text(
      "(c) Scarfy sprite by Eiden Marsal",
      SCREEN_WIDTH - 200,
      SCREEN_HEIGHT - 20,
      10,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_texture(scarfy);

  close_window();
}
