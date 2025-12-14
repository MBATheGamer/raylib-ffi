use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_3d, clear_background, close_window, disable_cursor, end_drawing,
    end_mode_3d, get_random_value, init_window, keyboard::is_key_pressed, set_target_fps,
    update_camera, window_should_close,
  },
  enums::{CameraMode, CameraProjection, KeyboardKey},
  model::{draw_cube, draw_cube_wires, draw_plane},
  shape::{draw_rectangle, draw_rectangle_lines},
  structs::{Camera3D, Color, Vector2, Vector3},
  text::draw_text,
  texture::fade,
};

const MAX_COLUMNS: usize = 20;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera first person",
  );

  let mut camera = Camera3D {
    position: Vector3 {
      x: 0.0,
      y: 2.0,
      z: 4.0,
    },
    target: Vector3 {
      x: 0.0,
      y: 2.0,
      z: 0.0,
    },
    up: Vector3 {
      x: 0.0,
      y: 1.0,
      z: 0.0,
    },
    fovy: 60.0,
    projection: CameraProjection::Perspective,
  };

  let mut camera_mode = CameraMode::FirstPerson;

  let mut heights: Vec<f32> = vec![];
  let mut positions: Vec<Vector3> = vec![];
  let mut colors: Vec<Color> = vec![];

  for i in 0..MAX_COLUMNS {
    heights.push(get_random_value(1, 12) as f32);
    positions.push(Vector3 {
      x: get_random_value(-15, 15) as f32,
      y: heights[i] / 2.0,
      z: get_random_value(-15, 15) as f32,
    });
    colors.push(Color {
      red: get_random_value(20, 255) as u8,
      green: get_random_value(10, 55) as u8,
      blue: 30,
      alpha: 255,
    });
  }

  disable_cursor();

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyOne) {
      camera_mode = CameraMode::Free;
      camera.up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
      };
    }

    if is_key_pressed(KeyboardKey::KeyTwo) {
      camera_mode = CameraMode::FirstPerson;
      camera.up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
      };
    }

    if is_key_pressed(KeyboardKey::KeyThree) {
      camera_mode = CameraMode::ThirdPerson;
      camera.up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
      };
    }

    if is_key_pressed(KeyboardKey::KeyFour) {
      camera_mode = CameraMode::Orbital;
      camera.up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
      };
    }

    if is_key_pressed(KeyboardKey::KeyP) {
      if camera.projection == CameraProjection::Perspective {
        camera_mode = CameraMode::ThirdPerson;

        camera.position = Vector3 {
          x: 0.0,
          y: 2.0,
          z: -100.0,
        };
        camera.target = Vector3 {
          x: 0.0,
          y: 2.0,
          z: 0.0,
        };
        camera.up = Vector3 {
          x: 0.0,
          y: 1.0,
          z: 0.0,
        };
        camera.projection = CameraProjection::Orthographic;
        camera.fovy = 20.0;
        camera.camera_yaw(-135.0f32.to_radians(), true);
        camera.camera_pitch(-45.0f32.to_radians(), true, true, false);
      } else if camera.projection == CameraProjection::Orthographic {
        camera_mode = CameraMode::ThirdPerson;
        camera.position = Vector3 {
          x: 0.0,
          y: 2.0,
          z: 10.0,
        };
        camera.target = Vector3 {
          x: 0.0,
          y: 2.0,
          z: 0.0,
        };
        camera.up = Vector3 {
          x: 0.0,
          y: 1.0,
          z: 0.0,
        };
        camera.projection = CameraProjection::Perspective;
        camera.fovy = 60.0;
      }
    }

    update_camera(&mut camera, camera_mode);

    // camera.update_camera_pro(
    //   Vector3 {
    //     x: (if is_key_down(KeyboardKey::KeyW) || is_key_down(KeyboardKey::KeyUp) {
    //       0.1
    //     } else {
    //       0.0
    //     }) * 0.1
    //       - (if is_key_down(KeyboardKey::KeyS) || is_key_down(KeyboardKey::KeyDown) {
    //         0.1
    //       } else {
    //         0.0
    //       }),
    //     y: (if is_key_down(KeyboardKey::KeyD) || is_key_down(KeyboardKey::KeyRight) {
    //       0.1
    //     } else {
    //       0.0
    //     }) - (if is_key_down(KeyboardKey::KeyA) || is_key_down(KeyboardKey::KeyLeft) {
    //       0.1
    //     } else {
    //       0.0
    //     }),
    //     z: 0.0,
    //   },
    //   Vector3 {
    //     x: get_mouse_delta().x * 0.05,
    //     y: get_mouse_delta().y * 0.05,
    //     z: 0.0,
    //   },
    //   get_mouse_wheel_move() * 2.0,
    // );

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);

    draw_plane(
      Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      },
      Vector2 { x: 32.0, y: 32.0 },
      colors::LIGHTGRAY,
    );
    draw_cube(
      Vector3 {
        x: -16.0,
        y: 2.5,
        z: 0.0,
      },
      1.0,
      5.0,
      32.0,
      colors::BLUE,
    );
    draw_cube(
      Vector3 {
        x: 16.0,
        y: 2.5,
        z: 0.0,
      },
      1.0,
      5.0,
      32.0,
      colors::LIME,
    );
    draw_cube(
      Vector3 {
        x: 0.0,
        y: 2.5,
        z: 16.0,
      },
      32.0,
      5.0,
      1.0,
      colors::GOLD,
    );

    for i in 0..MAX_COLUMNS {
      draw_cube(positions[i], 2.0, heights[i], 2.0, colors[i]);
      draw_cube_wires(positions[i], 2.0, heights[i], 2.0, colors::MAROON);
    }

    if camera_mode == CameraMode::ThirdPerson {
      draw_cube(camera.target, 0.5, 0.5, 0.5, colors::PURPLE);
      draw_cube_wires(camera.target, 0.5, 0.5, 0.5, colors::DARKPURPLE);
    }

    end_mode_3d();

    draw_rectangle(5, 5, 330, 100, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(5, 5, 330, 100, colors::BLUE);

    draw_text("Camera controls:", 15, 15, 10, colors::BLACK);
    draw_text(
      "- Move keys: W, A, S, D, Space, Left-Ctrl",
      15,
      30,
      10,
      colors::BLACK,
    );
    draw_text(
      "- Look around: arrow keys or mouse",
      15,
      45,
      10,
      colors::BLACK,
    );
    draw_text("- Camera mode keys: 1, 2, 3, 4", 15, 60, 10, colors::BLACK);
    draw_text(
      "- Zoom keys: num-plus, num-minus or mouse scroll",
      15,
      75,
      10,
      colors::BLACK,
    );
    draw_text("- Camera projection key: P", 15, 90, 10, colors::BLACK);

    draw_rectangle(600, 5, 195, 100, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(600, 5, 195, 100, colors::BLUE);

    draw_text("Camera status:", 610, 15, 10, colors::BLACK);
    draw_text(
      &format!(
        "- Mode: {}",
        match camera_mode {
          CameraMode::Free => "FREE",
          CameraMode::Orbital => "ORBITAL",
          CameraMode::FirstPerson => "FIRST_PERSON",
          CameraMode::ThirdPerson => "THIRD_PERSON",
          CameraMode::Custom => "CUSTOM",
        }
      ),
      610,
      30,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!(
        "- Projection: {}",
        match camera.projection {
          CameraProjection::Perspective => "PERSPECTIVE",
          CameraProjection::Orthographic => "ORTHOGRAPHIC",
        }
      ),
      610,
      45,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!(
        "- Position: ({:06.3}, {:06.3}, {:06.3})",
        camera.position.x, camera.position.y, camera.position.z
      ),
      610,
      60,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!(
        "- Target: ({:06.3}, {:06.3}, {:06.3})",
        camera.target.x, camera.target.y, camera.target.z
      ),
      610,
      75,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!(
        "- Up: ({:06.3}, {:06.3}, {:06.3})",
        camera.up.x, camera.up.y, camera.up.z
      ),
      610,
      90,
      10,
      colors::BLACK,
    );

    end_drawing();
  }

  close_window();
}
