fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d picking",
  );

  // Define the camera to look into our 3d world
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
    y: 1.0,
    z: 0.0,
  };
  let cube_size = Vector3 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
  };

  let ray = Ray::default();
  let collision = RayCollision::default();

  set_target_fps(60);

  while !window_should_close() {
    if is_cursor_hidden() {
      update_camera(&mut camera, CameraMode::FirstPerson);
    }

    if is_mouse_button_pressed(MouseButton::Right) {
      if is_cursor_hidden() {
        enable_cursor();
      } else {
        disable_cursor();
      }
    }

    if is_mouse_button_pressed(MouseButton::Left) {
      if !collision.hit {
        ray = get_screen_to_world_ray(get_mouse_position(), camera);

        collision = get_ray_collision_box(
          ray,
          BoundingBox {
            min: Vector3 {
              x: cube_position.x - cube_size.x / 2,
              y: cube_position.y - cube_size.y / 2,
              z: cube_position.z - cube_size.z / 2,
            },
            max: Vector3 {
              x: cube_position.x + cube_size.x / 2,
              y: cube_position.y + cube_size.y / 2,
              z: cube_position.z + cube_size.z / 2,
            },
          },
        );
      } else {
        collision.hit = false;
      }
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);

    if collision.hit {
      draw_cube(
        cube_position,
        cube_size.x,
        cube_size.y,
        cube_size.z,
        colors::RED,
      );
      draw_cube_wires(
        cube_position,
        cube_size.x,
        cube_size.y,
        cube_size.z,
        colors::MAROON,
      );

      draw_cube_wires(
        cube_position,
        cube_size.x + 0.2,
        cube_size.y + 0.2,
        cube_size.z + 0.2,
        colors::GREEN,
      );
    } else {
      draw_cube(
        cube_position,
        cube_size.x,
        cube_size.y,
        cube_size.z,
        colors::GRAY,
      );
      draw_cube_wires(
        cube_position,
        cube_size.x,
        cube_size.y,
        cube_size.z,
        colors::DARKGRAY,
      );
    }

    draw_ray(ray, colors::MAROON);
    draw_grid(10, 1.0);

    end_mode_3d();

    draw_text(
      "Try clicking on the box with your mouse!",
      240,
      10,
      20,
      colors::DARKGRAY,
    );

    if collision.hit {
      draw_text(
        "BOX SELECTED",
        (SCREEN_WIDTH - measure_text("BOX SELECTED", 30)) / 2,
        (SCREEN_HEIGHT as f32 * 0.1) as i32,
        30,
        colors::GREEN,
      );
    }

    draw_text(
      "Right click mouse to toggle camera controls",
      10,
      430,
      10,
      colors::GRAY,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
