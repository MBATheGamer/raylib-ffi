use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    touch::{get_touch_point_count, get_touch_position},
    window_should_close,
  },
  shape::draw_circle_v,
  structs::Vector2,
  text::draw_text,
};

fn main() {
  const MAX_TOUCH_POINTS: usize = 10;
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - input multitouch",
  );

  let mut touch_positions: [Vector2; MAX_TOUCH_POINTS] = [Default::default(); 10];

  set_target_fps(60);

  while !window_should_close() {
    let mut touch_count = get_touch_point_count() as usize;
    if touch_count > MAX_TOUCH_POINTS {
      touch_count = MAX_TOUCH_POINTS;
    }
    for i in 0..touch_count {
      touch_positions[i] = get_touch_position(i as i32);
    }
    begin_drawing();

    clear_background(colors::RAYWHITE);

    for i in 0..touch_count {
      if (touch_positions[i].x > 0.0) && (touch_positions[i].y > 0.0) {
        draw_circle_v(touch_positions[i], 34.0, colors::ORANGE);
        draw_text(
          &format!("{}", i),
          touch_positions[i].x as i32 - 10,
          touch_positions[i].y as i32 - 70,
          40,
          colors::BLACK,
        );
      }
    }

    draw_text(
      "touch the screen at multiple locations to get multiple balls",
      10,
      10,
      20,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  close_window();
}
