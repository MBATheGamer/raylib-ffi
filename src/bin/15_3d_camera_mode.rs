use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_3d, clear_background, close_window, end_drawing, end_mode_3d,
    init_window, set_target_fps, window_should_close,
  },
  enums::CameraProjection,
  model::{draw_cube, draw_cube_wires, draw_grid},
  structs::{Camera3D, Vector3},
  text::{draw_fps, draw_text},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera mode",
  );

  let camera = Camera3D {
    position: Vector3 {
      x: 0.0,
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

  set_target_fps(60);

  while !window_should_close() {
    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);

    draw_cube(cube_position, 2.0, 2.0, 2.0, colors::RED);
    draw_cube_wires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);

    draw_grid(10, 1.0);

    end_mode_3d();

    draw_text(
      "Welcome to the third dimension!",
      10,
      40,
      20,
      colors::DARKGRAY,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
