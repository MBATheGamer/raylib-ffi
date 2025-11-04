fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  const VIRTUAL_SCREEN_WIDTH: i32 = 160;
  const VIRTUAL_SCREEN_HEIGHT: i32 = 90;

  const VIRTUAL_RATIO: f32 = SCREEN_WIDTH as f32 / VIRTUAL_SCREEN_WIDTH as f32;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - smooth pixelperfect",
  );

  let world_space_camera = Camera2D {
    zoom: 1.0,
    ..Default::default()
  };

  let screen_space_camera = Camera2D {
    zoom: 1.0,
    ..Default::default()
  };

  let target = load_render_texture(VIRTUAL_SCREEN_WIDTH, VIRTUAL_SCREEN_HEIGHT);

  let rec01 = Rectangle {
    x: 70.0,
    y: 35.0,
    width: 20.0,
    height: 20.0,
  };
  let rec02 = Rectangle {
    x: 90.0,
    y: 55.0,
    width: 30.0,
    height: 10.0,
  };
  let rec03 = Rectangle {
    x: 80.0,
    y: 65.0,
    width: 15.0,
    height: 25.0,
  };

  let source_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: target.texture.width as f32,
    height: -target.texture.height as f32,
  };
  let dest_rec = Rectangle {
    x: -VIRTUAL_RATIO,
    y: -VIRTUAL_RATIO,
    width: SCREEN_WIDTH as f32 + (VIRTUAL_RATIO * 2.0),
    height: SCREEN_HEIGHT as f32 + (VIRTUAL_RATIO * 2.0),
  };

  let origin = Vector2 { x: 0.0, y: 0.0 };

  let rotation = 0.0;

  set_target_fps(60);

  while !window_should_close() {
    rotation += 60.0 * get_frame_time();

    let camera_x = get_time().sin() * 50.0 - 10.0;
    let camera_y = get_time().cos() * 30.0;

    screen_space_camera.target = Vector2 {
      x: camera_x as f32,
      y: camera_y as f32,
    };

    world_space_camera.target.x = screen_space_camera.target.x.trunc();
    screen_space_camera.target.x -= world_space_camera.target.x;
    screen_space_camera.target.x *= VIRTUAL_RATIO;

    world_space_camera.target.y = screen_space_camera.target.y.trunc();
    screen_space_camera.target.y -= world_space_camera.target.y;
    screen_space_camera.target.y *= VIRTUAL_RATIO;

    begin_texture_mode(target);
    clear_background(colors::RAYWHITE);

    begin_mode_2d(world_space_camera);
    draw_rectangle_pro(rec01, origin, rotation, colors::BLACK);
    draw_rectangle_pro(rec02, origin, -rotation, colors::RED);
    draw_rectangle_pro(rec03, origin, rotation + 45.0, colors::BLUE);
    end_mode_2d();
    end_texture_mode();

    begin_drawing();
    clear_background(colors::RED);

    begin_mode_2d(screen_space_camera);
    draw_texture_pro(
      target.texture,
      source_rec,
      dest_rec,
      origin,
      0.0,
      colors::WHITE,
    );
    end_mode_2d();

    draw_text(
      &format!("Screen resolution: {}x{}", SCREEN_WIDTH, SCREEN_HEIGHT),
      10,
      10,
      20,
      colors::DARKBLUE,
    );
    draw_text(
      &format!(
        "World resolution: {}x{}",
        VIRTUAL_SCREEN_WIDTH, VIRTUAL_SCREEN_HEIGHT
      ),
      10,
      40,
      20,
      colors::DARKGREEN,
    );
    draw_fps(get_screen_width() - 95, 10);
    end_drawing();
  }

  unload_render_texture(target);

  close_window();
}
