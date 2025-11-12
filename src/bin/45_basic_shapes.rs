use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  shape::{
    draw_circle, draw_circle_gradient, draw_circle_lines, draw_ellipse, draw_ellipse_lines,
    draw_line, draw_poly, draw_poly_lines, draw_poly_lines_ex, draw_rectangle,
    draw_rectangle_gradient_v, draw_rectangle_lines, draw_triangle, draw_triangle_lines,
  },
  structs::Vector2,
  text::draw_text,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - basic shapes",
  );

  let mut rotation = 0.0;

  set_target_fps(60);

  while !window_should_close() {
    rotation += 0.2;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_text(
      "some basic shapes available on raylib",
      20,
      20,
      20,
      colors::DARKGRAY,
    );

    draw_circle(SCREEN_WIDTH / 5, 120, 35.0, colors::DARKBLUE);
    draw_circle_gradient(SCREEN_WIDTH / 5, 220, 60.0, colors::GREEN, colors::SKYBLUE);
    draw_circle_lines(SCREEN_WIDTH / 5, 340, 80.0, colors::DARKBLUE);
    draw_ellipse(SCREEN_WIDTH / 5, 120, 25.0, 20.0, colors::YELLOW);
    draw_ellipse_lines(SCREEN_WIDTH / 5, 120, 30.0, 25.0, colors::YELLOW);

    draw_rectangle(SCREEN_WIDTH / 4 * 2 - 60, 100, 120, 60, colors::RED);
    draw_rectangle_gradient_v(
      SCREEN_WIDTH / 4 * 2 - 90,
      170,
      180,
      130,
      colors::MAROON,
      colors::GOLD,
    );
    draw_rectangle_lines(SCREEN_WIDTH / 4 * 2 - 40, 320, 80, 60, colors::ORANGE);

    draw_triangle(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 80.0,
      },
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0 - 60.0,
        y: 150.0,
      },
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0 + 60.0,
        y: 150.0,
      },
      colors::VIOLET,
    );

    draw_triangle_lines(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 160.0,
      },
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0 - 20.0,
        y: 230.0,
      },
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0 + 20.0,
        y: 230.0,
      },
      colors::DARKBLUE,
    );

    draw_poly(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 330.0,
      },
      6,
      80.0,
      rotation,
      colors::BROWN,
    );
    draw_poly_lines(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 330.0,
      },
      6,
      90.0,
      rotation,
      colors::BROWN,
    );
    draw_poly_lines_ex(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 330.0,
      },
      6,
      85.0,
      rotation,
      6.0,
      colors::BEIGE,
    );

    draw_line(18, 42, SCREEN_WIDTH - 18, 42, colors::BLACK);
    end_drawing();
  }

  close_window();
}
