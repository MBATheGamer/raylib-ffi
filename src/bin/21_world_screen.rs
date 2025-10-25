fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - world screen",
  );

  let camera = Camera3D {
    position: Vector3 {
      x: 10.0,
      y: 10.0,
      z: 10.0,
    },
    target: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    up: Vector3 {
      x: 0.0,
      y: 1.0,
      z: 0.0,
    },
    fovy: 45.0,
    projection: CameraProjection::Perspective,
  };

  let cube_position = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };

  disable_cursor();

  set_target_fps(60);

  while !window_should_close() {
    update_camera(&mut camera, CameraMode::ThirdPerson);

    let cube_screen_position = get_world_to_screen(
      Vector3 {
        x: cube_position.x,
        y: cube_position.y + 2.5,
        z: cube_position.z,
      },
      camera,
    );

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);

    draw_cube(cube_position, 2.0, 2.0, 2.0, colors::RED);
    draw_cube_wires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);

    draw_grid(10, 1.0);

    end_mode_3d();

    draw_text(
      "Enemy: 100/100",
      cube_screen_position.x as i32 - measure_text("Enemy: 100/100", 20) / 2,
      cube_screen_position.y as i32,
      20,
      colors::BLACK,
    );

    draw_text(
      &format!(
        "Cube position in screen space coordinates: [{}, {}]",
        cube_screen_position.x as i32, cube_screen_position.y as i32
      ),
      10,
      10,
      20,
      colors::LIME,
    );
    draw_text(
      "Text 2d should be always on top of the cube",
      10,
      40,
      20,
      colors::GRAY,
    );

    end_drawing();
  }

  close_window();
}
