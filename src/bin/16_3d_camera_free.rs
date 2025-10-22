use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_3d, clear_background, close_window, disable_cursor, end_drawing,
    end_mode_3d, init_window, keyboard::is_key_pressed, set_target_fps, update_camera,
    window_should_close,
  },
  enums::{CameraMode, CameraProjection, KeyboardKey},
  model::{draw_cube, draw_cube_wires, draw_grid},
  shapes::{draw_rectangle, draw_rectangle_lines},
  structs::{Camera3D, Vector3},
  text::draw_text,
  texture::fade,
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera free",
  );

  let mut camera = Camera3D {
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
    update_camera(&mut camera, CameraMode::Free);

    if is_key_pressed(KeyboardKey::KeyZ) {
      camera.target = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
      };
    }

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);

    draw_cube(cube_position, 2.0, 2.0, 2.0, colors::RED);
    draw_cube_wires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);

    draw_grid(10, 1.0);

    end_mode_3d();

    draw_rectangle(10, 10, 320, 93, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(10, 10, 320, 93, colors::BLUE);

    draw_text("Free camera default controls:", 20, 20, 10, colors::BLACK);
    draw_text("- Mouse Wheel to Zoom in-out", 40, 40, 10, colors::DARKGRAY);
    draw_text("- Mouse Wheel Pressed to Pan", 40, 60, 10, colors::DARKGRAY);
    draw_text("- Z to zoom to (0, 0, 0)", 40, 80, 10, colors::DARKGRAY);

    end_drawing();
  }

  close_window();
}
