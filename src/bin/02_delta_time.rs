use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_fps, get_frame_time,
    init_window, keyboard::is_key_pressed, mouse::get_mouse_wheel_move, set_target_fps,
    window_should_close,
  },
  enums::KeyboardKey,
  shape::draw_circle_v,
  structs::Vector2,
  text::draw_text,
};

fn main() {
  let screen_width = 800;
  let screen_height = 450;

  init_window(
    screen_width,
    screen_height,
    "raylib [core] example - delta time",
  );

  let mut current_fps = 60;

  let mut delta_circle = Vector2 {
    x: 0.0,
    y: screen_height as f32 / 3.0,
  };

  let mut frame_circle = Vector2 {
    x: 0.0,
    y: screen_height as f32 * 2.0 / 3.0,
  };

  let speed = 10.0;
  let circle_radius = 32.0;

  set_target_fps(current_fps);

  while !window_should_close() {
    let mouse_wheel = get_mouse_wheel_move();

    if mouse_wheel != 0.0 {
      current_fps += mouse_wheel as i32;
      if current_fps < 0 {
        current_fps = 0;
      }
      set_target_fps(current_fps);
    }

    delta_circle.x += get_frame_time() * 6.0 * speed;
    frame_circle.x += 0.1 * speed;

    if delta_circle.x > screen_width as f32 {
      delta_circle.x = 0.0
    }

    if frame_circle.x > screen_width as f32 {
      frame_circle.x = 0.0
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      delta_circle.x = 0.0;
      frame_circle.x = 0.0;
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_circle_v(delta_circle, circle_radius, colors::RED);
    draw_circle_v(frame_circle, circle_radius, colors::BLUE);

    let fps_text = if current_fps <= 0 {
      format!("FPS: unlimited ({})", get_fps())
    } else {
      format!("FPS: {} (target: {})", get_fps(), current_fps)
    };

    draw_text(&fps_text, 10, 10, 20, colors::DARKGRAY);
    draw_text(
      &format!("Frame time: {}ms", get_frame_time()),
      10,
      30,
      20,
      colors::DARKGRAY,
    );
    draw_text(
      "Use the scroll wheel to change the fps limit, r to reset",
      10,
      50,
      20,
      colors::DARKGRAY,
    );

    draw_text(
      "FUNC: x += get_frame_time() * speed",
      10,
      90,
      20,
      colors::RED,
    );

    draw_text("FUNC: x += speed", 10, 240, 20, colors::BLUE);

    end_drawing();
  }

  close_window();
}
