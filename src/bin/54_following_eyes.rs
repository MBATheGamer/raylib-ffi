use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_screen_height,
    get_screen_width, init_window, mouse::get_mouse_position, set_target_fps, window_should_close,
  },
  shape::{check_collision_point_circle, draw_circle_v},
  structs::Vector2,
  text::draw_fps,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - following eyes",
  );

  let sclera_left_position = Vector2 {
    x: get_screen_width() as f32 / 2.0 - 100.0,
    y: get_screen_height() as f32 / 2.0,
  };
  let sclera_right_position = Vector2 {
    x: get_screen_width() as f32 / 2.0 + 100.0,
    y: get_screen_height() as f32 / 2.0,
  };
  let sclera_radius = 80.0;

  let iris_radius = 24;

  set_target_fps(60);

  while !window_should_close() {
    let mut iris_left_position = get_mouse_position();
    let mut iris_right_position = get_mouse_position();

    if !check_collision_point_circle(
      iris_left_position,
      sclera_left_position,
      sclera_radius - iris_radius as f32,
    ) {
      let dx = iris_left_position.x - sclera_left_position.x;
      let dy = iris_left_position.y - sclera_left_position.y;

      let angle = dy.atan2(dx);

      let dxx = (sclera_radius - iris_radius as f32) * angle.cos();
      let dyy = (sclera_radius - iris_radius as f32) * angle.sin();

      iris_left_position.x = sclera_left_position.x + dxx;
      iris_left_position.y = sclera_left_position.y + dyy;
    }

    if !check_collision_point_circle(
      iris_right_position,
      sclera_right_position,
      sclera_radius - iris_radius as f32,
    ) {
      let dx = iris_right_position.x - sclera_right_position.x;
      let dy = iris_right_position.y - sclera_right_position.y;

      let angle = dy.atan2(dx);

      let dxx = (sclera_radius - iris_radius as f32) * angle.cos();
      let dyy = (sclera_radius - iris_radius as f32) * angle.sin();

      iris_right_position.x = sclera_right_position.x + dxx;
      iris_right_position.y = sclera_right_position.y + dyy;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_circle_v(sclera_left_position, sclera_radius, colors::LIGHTGRAY);
    draw_circle_v(iris_left_position, iris_radius as f32, colors::BROWN);
    draw_circle_v(iris_left_position, 10.0, colors::BLACK);

    draw_circle_v(sclera_right_position, sclera_radius, colors::LIGHTGRAY);
    draw_circle_v(iris_right_position, iris_radius as f32, colors::DARKGREEN);
    draw_circle_v(iris_right_position, 10.0, colors::BLACK);

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
