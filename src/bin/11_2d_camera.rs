fn main() {
  const MAX_BUILDINGS: i32 = 100;
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 2d camera",
  );

  let player = Rectangle {
    x: 400.0,
    y: 280.0,
    width: 40.0,
    height: 40.0,
  };
  let buildings: Vec<Rectangle> = vec![];
  let build_colors: Vec<Color> = vec![];

  let spacing = 0.0;

  for i in 0..MAX_BUILDINGS {
    let height = get_random_value(100, 800);
    buildings.push(Rectangle {
      x: -6000.0 + spacing,
      y: SCREEN_HEIGHT - 130.0 - height,
      width: get_random_value(50, 200),
      height: height,
    });

    spacing += buildings[i as usize].width;

    build_colors.push(Color {
      red: get_random_value(200, 240),
      green: get_random_value(200, 240),
      blue: get_random_value(200, 250),
      alpha: 255,
    });
  }

  let camera: Camera2D = { 0 };
  camera.target = Vector2 {
    x: player.x + 20.0,
    y: player.y + 20.0,
  };
  camera.offset = Vector2 {
    x: SCREEN_WIDTH / 2.0,
    y: SCREEN_HEIGHT / 2.0,
  };
  camera.rotation = 0.0;
  camera.zoom = 1.0;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeyRight) {
      player.x += 2.0;
    } else if is_key_down(KeyboardKey::KeyLeft) {
      player.x -= 2.0;
    }

    camera.target = Vector2 {
      x: player.x + 20.0,
      y: player.y + 20.0,
    };

    if is_key_down(KeyboardKey::KeyA) {
      camera.rotation -= 1;
    } else if is_key_down(KeyboardKey::KeyS) {
      camera.rotation += 1;
    }

    if camera.rotation > 40 {
      camera.rotation = 40;
    } else if camera.rotation < -40 {
      camera.rotation = -40;
    }

    camera.zoom = (camera.zoom.ln() + get_mouse_wheel_move() * 0.1).exp();

    if camera.zoom > 3.0 {
      camera.zoom = 3.0;
    } else if camera.zoom < 0.1 {
      camera.zoom = 0.1;
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      camera.zoom = 1.0;
      camera.rotation = 0.0;
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_2d(camera);

    draw_rectangle(-6000, 320, 13000, 8000, colors::DARKGRAY);

    for i in 0..MAX_BUILDINGS {
      draw_rectangle_rec(buildings[i], build_colors[i]);
    }

    draw_rectangle_rec(player, colors::RED);

    draw_line(
      camera.target.x,
      -SCREEN_HEIGHT * 10,
      camera.target.x,
      SCREEN_HEIGHT * 10,
      colors::GREEN,
    );
    draw_line(
      -SCREEN_WIDTH * 10,
      camera.target.y,
      SCREEN_WIDTH * 10,
      camera.target.y,
      colors::GREEN,
    );

    end_mod_2d();

    draw_text("SCREEN AREA", 640, 10, 20, colors::RED);

    draw_rectangle(0, 0, SCREEN_WIDTH, 5, colors::RED);
    draw_rectangle(0, 5, 5, SCREEN_HEIGHT - 10, colors::RED);
    draw_rectangle(SCREEN_WIDTH - 5, 5, 5, SCREEN_HEIGHT - 10, colors::RED);
    draw_rectangle(0, SCREEN_HEIGHT - 5, SCREEN_WIDTH, 5, colors::RED);

    draw_rectangle(10, 10, 250, 113, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(10, 10, 250, 113, colors::BLUE);

    draw_text("Free 2d camera controls:", 20, 20, 10, colors::BLACK);
    draw_text("- Right/Left to move Offset", 40, 40, 10, colors::DARKGRAY);
    draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, colors::DARKGRAY);
    draw_text("- A / S to Rotate", 40, 80, 10, colors::DARKGRAY);
    draw_text(
      "- R to reset Zoom and Rotation",
      40,
      100,
      10,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  close_window();
}
