use std::env;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_screen_height,
    get_screen_width, get_time, init_window, keyboard::is_key_pressed, poll_input_events,
    swap_screen_buffer, wait_time, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_circle, draw_rectangle},
  text::draw_text,
};

fn main() {
  let platform = match env::var("PLATFORM") {
    Ok(platform) => platform,
    _ => format!("DESKTOP"),
  };

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - custom frame control",
  );

  let mut previous_time = get_time();
  let mut delta_time = 0.0;

  let mut time_counter = 0.0;
  let mut position = 0.0;
  let mut pause = false;

  let mut target_fps = 60;

  while !window_should_close() {
    if platform != "WEB" {
      poll_input_events();
    }
    if is_key_pressed(KeyboardKey::KeySpace) {
      pause = !pause;
    }

    if is_key_pressed(KeyboardKey::KeyUp) {
      target_fps += 20;
    } else if is_key_pressed(KeyboardKey::KeyDown) {
      target_fps -= 20;
    }

    if target_fps < 0 {
      target_fps = 0;
    }

    if !pause {
      position += 200.0 * delta_time;
      if position as i32 >= get_screen_width() {
        position = 0.0;
      }
      time_counter += delta_time;
    }

    if platform == "WEB" {
      poll_input_events();
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    for i in 0..(get_screen_width() / 200) {
      draw_rectangle(200 * i, 0, 1, get_screen_height(), colors::SKYBLUE);
    }

    draw_circle(
      position as i32,
      get_screen_height() / 2 - 25,
      50.0,
      colors::RED,
    );

    draw_text(
      &format!("{:.0} ms", time_counter * 1000.0),
      position as i32 - 40,
      get_screen_height() / 2 - 100,
      20,
      colors::MAROON,
    );
    draw_text(
      &format!("PosX: {}", position as i32),
      position as i32 - 50,
      get_screen_height() / 2 + 40,
      20,
      colors::BLACK,
    );

    draw_text(
      "Circle is moving at a constant 200 pixels/sec,\nindependently of the frame rate.",
      10,
      10,
      20,
      colors::DARKGRAY,
    );
    draw_text(
      "PRESS SPACE to PAUSE MOVEMENT",
      10,
      get_screen_height() - 60,
      20,
      colors::GRAY,
    );
    draw_text(
      "PRESS UP | DOWN to CHANGE TARGET FPS",
      10,
      get_screen_height() - 30,
      20,
      colors::GRAY,
    );
    draw_text(
      &format!("TARGET FPS: {}", target_fps),
      get_screen_width() - 220,
      10,
      20,
      colors::LIME,
    );
    if delta_time != 0.0 {
      draw_text(
        &format!("CURRENT FPS: {}", (1.0 / delta_time) as i32),
        get_screen_width() - 220,
        40,
        20,
        colors::GREEN,
      );
    }

    end_drawing();

    swap_screen_buffer();

    let mut current_time = get_time();
    let update_draw_time = current_time - previous_time;

    if target_fps > 0 {
      let waiting_time = (1.0 / target_fps as f64) - update_draw_time as f64;
      if waiting_time > 0.0 {
        wait_time(waiting_time);
        current_time = get_time();
        delta_time = (current_time - previous_time) as f32;
      }
    } else {
      delta_time = update_draw_time as f32;
    }

    previous_time = current_time;
  }

  close_window();
}
