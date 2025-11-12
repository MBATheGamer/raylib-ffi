fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - basic shapes",
  );

  let rotation = 0.0;

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
    draw_circle_gradient(SCREEN_WIDTH / 5, 220, 60, colors::GREEN, colors::SKYBLUE);
    draw_circle_lines(SCREEN_WIDTH / 5, 340, 80, colors::DARKBLUE);
    draw_ellipse(SCREEN_WIDTH / 5, 120, 25, 20, colors::YELLOW);
    draw_ellipse_lines(SCREEN_WIDTH / 5, 120, 30, 25, colors::YELLOW);

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
      80,
      rotation,
      colors::BROWN,
    );
    draw_poly_lines(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3,
        y: 330.0,
      },
      6,
      90,
      rotation,
      colors::BROWN,
    );
    draw_poly_lines_ex(
      Vector2 {
        x: SCREEN_WIDTH as f32 / 4.0 * 3.0,
        y: 330.0,
      },
      6,
      85,
      rotation,
      6,
      colors::BEIGE,
    );

    draw_line(18, 42, SCREEN_WIDTH - 18, 42, colors::BLACK);
    end_drawing();
  }

  close_window();
}
