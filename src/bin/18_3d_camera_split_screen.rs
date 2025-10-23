fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera split screen",
  );

  let mut camera_player1 = Camera3D {
    fovy: 45.0,
    up: Vector3 {
      x: 0.0,
      y: 1.0,
      z: 0.0,
    },
    target: Vector3 {
      y: 1.0,
      ..Default::default()
    },
    position: Vector3 {
      y: 1.0,
      z: -3.0,
      ..Default::default()
    },
    ..Default::default()
  };

  let screen_player1 = load_render_texture(SCREEN_WIDTH / 2, SCREEN_HEIGHT);

  let mut camera_player2 = Camera3D {
    fovy: 45.0,
    up: Vector3 {
      y: 1.0,
      ..Default::default()
    },
    target: Vector3 {
      y: 3.0,
      ..Default::default()
    },
    position: Vector3 {
      x: -3.0,
      y: 3.0,
      z: 0.0,
    },
    ..Default::default()
  };

  let screen_player2 = load_render_texture(SCREEN_WIDTH / 2, SCREEN_HEIGHT);

  let split_screen_rect = Rectangle {
    x: 0.0,
    y: 0.0,
    width: screen_player1.texture.width as f32,
    height: -screen_player1.texture.height as f32,
  };

  let count = 5;
  let spacing = 4.0;

  set_target_fps(60);

  while !window_should_close() {
    let offset_this_frame = 10.0 * get_frame_time();

    if is_key_down(KeyboardKey::KeyW) {
      camera_player1.position.z += offset_this_frame;
      camera_player1.target.z += offset_this_frame;
    } else if is_key_down(KeyboardKey::KeyS) {
      camera_player1.position.z -= offset_this_frame;
      camera_player1.target.z -= offset_this_frame;
    }

    if is_key_down(KeyboardKey::KeyUp) {
      camera_player2.position.x += offset_this_frame;
      camera_player2.target.x += offset_this_frame;
    } else if is_key_down(KeyboardKey::KeyDown) {
      camera_player2.position.x -= offset_this_frame;
      camera_player2.target.x -= offset_this_frame;
    }

    begin_texture_mode(screen_player1);
    clear_background(colors::SKYBLUE);

    begin_mode_3d(camera_player1);
    draw_plane(
      Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      Vector2 { x: 50.0, y: 50.0 },
      colors::BEIGE,
    );

    let mut x = -count as f32 * spacing;
    while x <= count as f32 * spacing {
      let mut z = -count as f32 * spacing;
      while z <= count as f32 * spacing {
        draw_cube(Vector3 { x, y: 1.5, z }, 1.0, 1.0, 1.0, colors::LIME);
        draw_cube(Vector3 { x, y: 0.5, z }, 0.25, 1.0, 0.25, colors::BROWN);
        z += spacing;
      }

      x += spacing;
    }

    draw_cube(camera_player1.position, 1.0, 1.0, 1.0, colors::RED);
    draw_cube(camera_player2.position, 1.0, 1.0, 1.0, colors::BLUE);
    end_mode_3d();

    draw_rectangle(
      0,
      0,
      get_screen_width() / 2,
      40,
      fade(colors::RAYWHITE, 0.8),
    );
    draw_text("PLAYER1: W/S to move", 10, 10, 20, colors::MAROON);
    end_texture_mode();

    begin_texture_mode(screen_player2);
    clear_background(colors::SKYBLUE);

    begin_mode_3d(camera_player2);
    draw_plane(
      Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      Vector2 { x: 50.0, y: 50.0 },
      colors::BEIGE,
    );

    let mut x = -count as f32 * spacing;
    while x <= count as f32 * spacing {
      let mut z = -count as f32 * spacing;
      while z <= count as f32 * spacing {
        draw_cube(Vector3 { x, y: 1.5, z }, 1.0, 1.0, 1.0, colors::LIME);
        draw_cube(Vector3 { x, y: 0.5, z }, 0.25, 1.0, 0.25, colors::BROWN);
        z += spacing;
      }

      x += spacing;
    }

    draw_cube(camera_player1.position, 1.0, 1.0, 1.0, colors::RED);
    draw_cube(camera_player2.position, 1.0, 1.0, 1.0, colors::BLUE);

    end_mode_3d();

    draw_rectangle(
      0,
      0,
      get_screen_width() / 2,
      40,
      fade(colors::RAYWHITE, 0.8),
    );
    draw_text("PLAYER2: UP/DOWN to move", 10, 10, 20, colors::DARKBLUE);

    end_texture_mode();

    begin_drawing();
    clear_background(colors::BLACK);

    draw_texture_rec(
      screen_player1.texture,
      split_screen_rect,
      Vector2 { x: 0.0, y: 0.0 },
      colors::WHITE,
    );
    draw_texture_rec(
      screen_player2.texture,
      split_screen_rect,
      Vector2 {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: 0.0,
      },
      colors::WHITE,
    );

    draw_rectangle(
      get_screen_width() / 2 - 2,
      0,
      4,
      get_screen_height(),
      colors::LIGHTGRAY,
    );
    end_drawing();
  }

  unload_render_texture(screen_player1);
  unload_render_texture(screen_player2);

  close_window();
}
