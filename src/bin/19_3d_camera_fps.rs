use std::{
  f32::consts::PI,
  sync::{LazyLock, Mutex},
};

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_3d, clear_background, close_window, disable_cursor, end_drawing,
    end_mode_3d, get_frame_time, init_window,
    keyboard::{is_key_down, is_key_pressed},
    mouse::get_mouse_delta,
    set_target_fps, window_should_close,
  },
  enums::{CameraProjection, KeyboardKey},
  math::lerp,
  model::{draw_cube_v, draw_cube_wires_v, draw_plane, draw_sphere},
  shape::{draw_rectangle, draw_rectangle_lines},
  structs::{Camera3D, Color, Vector2, Vector3},
  text::draw_text,
  texture::fade,
};

const GRAVITY: f32 = 32.0;
const MAX_SPEED: f32 = 20.0;
const CROUCH_SPEED: f32 = 5.0;
const JUMP_FORCE: f32 = 12.0;
const MAX_ACCEL: f32 = 150.0;

const FRICTION: f32 = 0.86;

const AIR_DRAG: f32 = 0.98;

const CONTROL: f32 = 15.0;
const CROUCH_HEIGHT: f32 = 0.0;
const STAND_HEIGHT: f32 = 1.0;
const BOTTOM_HEIGHT: f32 = 0.5;

const NORMALIZE_INPUT: bool = false;

#[derive(Default)]
struct Body {
  position: Vector3,
  velocity: Vector3,
  dir: Vector3,
  is_grounded: bool,
}

const SENSITIVITY: Vector2 = Vector2 { x: 0.001, y: 0.001 };

static PLAYER: LazyLock<Mutex<Body>> = LazyLock::new(|| {
  return Mutex::new(Body {
    position: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    velocity: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    dir: Vector3 {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    is_grounded: false,
  });
});
static LOOK_ROTATION: LazyLock<Mutex<Vector2>> = LazyLock::new(|| {
  return Mutex::new(Vector2::default());
});
static HEAD_TIMER: Mutex<f32> = Mutex::new(0.0);
static WALK_LERP: Mutex<f32> = Mutex::new(0.0);
static HEAD_LERP: Mutex<f32> = Mutex::new(STAND_HEIGHT);
static LEAN: LazyLock<Mutex<Vector2>> = LazyLock::new(|| {
  return Mutex::new(Vector2::default());
});

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera fps",
  );

  let mut head_lerp = HEAD_LERP
    .lock()
    .expect("expecting HEAD_LERP was intialized");

  let mut player = PLAYER.lock().expect("expecting PLAYER was intialized");

  let mut camera = Camera3D {
    fovy: 60.0,
    projection: CameraProjection::Perspective,
    position: Vector3 {
      x: player.position.x,
      y: player.position.y + (BOTTOM_HEIGHT + *head_lerp),
      z: player.position.z,
    },
    target: Vector3::default(),
    up: Vector3::default(),
  };

  update_camera_fps(&mut camera);

  disable_cursor();

  set_target_fps(60);

  while !window_should_close() {
    let mouse_delta = get_mouse_delta();
    // update_camera_fps
    let mut look_rotation = LOOK_ROTATION
      .lock()
      .expect("expecting LOOK_ROTATION was intialized");
    // update_camera_fps
    let mut head_timer = HEAD_TIMER
      .lock()
      .expect("expecting HEAD_TIMER was intialized");
    // update_camera_fps
    let mut walk_lerp = WALK_LERP
      .lock()
      .expect("expecting WALK_LERP was intialized");
    // update_camera_fps
    let mut lean = LEAN.lock().expect("expecting LEAN was intialized");

    look_rotation.x -= mouse_delta.x * SENSITIVITY.x;
    look_rotation.y += mouse_delta.y * SENSITIVITY.y;

    let sideway = if is_key_down(KeyboardKey::KeyD) {
      1.0
    } else if is_key_down(KeyboardKey::KeyA) {
      -1.0
    } else {
      0.0
    };
    let forward = if is_key_down(KeyboardKey::KeyW) {
      1.0
    } else if is_key_down(KeyboardKey::KeyS) {
      -1.0
    } else {
      0.0
    };
    let crouching = is_key_down(KeyboardKey::KeyLeftControl);
    update_body(
      &mut player,
      look_rotation.x,
      sideway,
      forward,
      is_key_pressed(KeyboardKey::KeySpace),
      crouching,
    );

    let delta = get_frame_time();
    *head_lerp = lerp(
      *head_lerp,
      if crouching {
        CROUCH_HEIGHT
      } else {
        STAND_HEIGHT
      },
      20.0 * delta,
    );
    camera.position = Vector3 {
      x: player.position.x,
      y: player.position.y + (BOTTOM_HEIGHT + *head_lerp),
      z: player.position.z,
    };

    if player.is_grounded && (forward != 0.0 || sideway != 0.0) {
      *head_timer += delta * 3.0;
      *walk_lerp = lerp(*walk_lerp, 1.0, 10.0 * delta);
      camera.fovy = lerp(camera.fovy, 55.0, 5.0 * delta);
    } else {
      *walk_lerp = lerp(*walk_lerp, 0.0, 10.0 * delta);
      camera.fovy = lerp(camera.fovy, 60.0, 5.0 * delta);
    }

    lean.x = lerp(lean.x, sideway * 0.02, 10.0 * delta);
    lean.y = lerp(lean.y, forward * 0.015, 10.0 * delta);

    drop(look_rotation);
    drop(lean);
    drop(head_timer);
    drop(walk_lerp);

    update_camera_fps(&mut camera);

    begin_drawing();

    clear_background(colors::RAYWHITE);

    begin_mode_3d(camera);
    draw_level();
    end_mode_3d();

    draw_rectangle(5, 5, 330, 75, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(5, 5, 330, 75, colors::BLUE);

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
    draw_text(
      &format!(
        "- Velocity Len: ({:06.3})",
        Vector2 {
          x: player.velocity.x,
          y: player.velocity.z
        }
        .length()
      ),
      15,
      60,
      10,
      colors::BLACK,
    );

    end_drawing();
  }

  close_window();
}

fn update_body(
  body: &mut Body,
  rot: f32,
  side: f32,
  forward: f32,
  jump_pressed: bool,
  crouch_hold: bool,
) {
  let mut input = Vector2 {
    x: side,
    y: -forward,
  };

  if NORMALIZE_INPUT {
    if side != 0.0 && forward != 0.0 {
      input = input.normalize();
    }
  }

  let delta = get_frame_time();

  if !body.is_grounded {
    body.velocity.y -= GRAVITY * delta;
  }

  if body.is_grounded && jump_pressed {
    body.velocity.y = JUMP_FORCE;
    body.is_grounded = false;
  }

  let front = Vector3 {
    x: rot.sin(),
    y: 0.0,
    z: rot.cos(),
  };
  let right = Vector3 {
    x: (-rot).cos(),
    y: 0.0,
    z: (-rot).sin(),
  };

  let desired_dir = Vector3 {
    x: input.x * right.x + input.y * front.x,
    y: 0.0,
    z: input.x * right.z + input.y * front.z,
  };
  body.dir = body.dir.lerp(desired_dir, CONTROL * delta);

  let decel = if body.is_grounded { FRICTION } else { AIR_DRAG };
  let mut hvel = Vector3 {
    x: body.velocity.x * decel,
    y: 0.0,
    z: body.velocity.z * decel,
  };

  let hvel_length = hvel.length();
  if hvel_length < (MAX_SPEED * 0.01) {
    hvel = Vector3::default();
  }

  let speed = hvel.dot_product(body.dir);

  let max_speed = if crouch_hold { CROUCH_SPEED } else { MAX_SPEED };
  let accel = (max_speed - speed).clamp(0.0, MAX_ACCEL * delta);
  hvel.x += body.dir.x * accel;
  hvel.z += body.dir.z * accel;

  body.velocity.x = hvel.x;
  body.velocity.z = hvel.z;

  body.position.x += body.velocity.x * delta;
  body.position.y += body.velocity.y * delta;
  body.position.z += body.velocity.z * delta;

  if body.position.y <= 0.0 {
    body.position.y = 0.0;
    body.velocity.y = 0.0;
    body.is_grounded = true;
  }
}

fn update_camera_fps(camera: &mut Camera3D) {
  const UP: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
  };
  const TARGET_OFFSET: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
  };

  let mut look_rotation = LOOK_ROTATION
    .lock()
    .expect("expecting LOOK_ROTATION was initialized");
  let lean = LEAN.lock().expect("expecting LEAN was initialized");
  let head_timer = HEAD_TIMER
    .lock()
    .expect("expecting HEAD_TIMER was initialized");
  let walk_lerp = WALK_LERP
    .lock()
    .expect("expecting WALK_LERP was initialzed");

  let yaw = TARGET_OFFSET.rotate_by_axis_angle(UP, look_rotation.x);

  let mut max_angle_up = UP.angle(yaw);
  max_angle_up -= 0.001;
  if -look_rotation.y > max_angle_up {
    look_rotation.y = -max_angle_up;
  }

  let mut max_angle_down = (-UP).angle(yaw);
  max_angle_down *= -1.0;
  max_angle_down += 0.001;
  if -look_rotation.y < max_angle_down {
    look_rotation.y = -max_angle_down;
  }

  let right = yaw.cross_product(UP).normalize();

  let mut pitch_angle = -look_rotation.y - lean.y;
  pitch_angle = pitch_angle.clamp(-PI / 2.0 + 0.0001, PI / 2.0 - 0.0001);

  let pitch = yaw.rotate_by_axis_angle(right, pitch_angle);

  let head_sin = (*head_timer * PI).sin();
  let head_cos = (*head_timer * PI).cos();
  const STEP_ROTATION: f32 = 0.01;
  camera.up = UP.rotate_by_axis_angle(pitch, head_sin * STEP_ROTATION + lean.x);

  const BOB_SIDE: f32 = 0.1;
  const BOB_UP: f32 = 0.15;
  let mut bobbing = right.scale(head_sin * BOB_SIDE);
  bobbing.y = (head_cos * BOB_UP).abs();

  camera.position = camera.position + bobbing.scale(*walk_lerp);
  camera.target = camera.position + pitch;
}

fn draw_level() {
  const FLOOR_EXTENT: i32 = 25;
  const TILE_SIZE: f32 = 5.0;
  const TILE_COLOR1: Color = Color {
    red: 150,
    green: 200,
    blue: 200,
    alpha: 255,
  };

  for y in -FLOOR_EXTENT..FLOOR_EXTENT {
    for x in -FLOOR_EXTENT..FLOOR_EXTENT {
      if y & 1 != 0 && x & 1 != 0 {
        draw_plane(
          Vector3 {
            x: x as f32 * TILE_SIZE,
            y: 0.0,
            z: y as f32 * TILE_SIZE,
          },
          Vector2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
          },
          TILE_COLOR1,
        );
      } else if y & 1 == 0 && x & 1 == 0 {
        draw_plane(
          Vector3 {
            x: x as f32 * TILE_SIZE,
            y: 0.0,
            z: y as f32 * TILE_SIZE,
          },
          Vector2 {
            x: TILE_SIZE,
            y: TILE_SIZE,
          },
          colors::LIGHTGRAY,
        );
      }
    }
  }

  const TOWER_SIZE: Vector3 = Vector3 {
    x: 16.0,
    y: 32.0,
    z: 16.0,
  };
  const TOWER_COLOR: Color = Color {
    red: 150,
    green: 200,
    blue: 200,
    alpha: 255,
  };

  let mut tower_pos = Vector3 {
    x: 16.0,
    y: 16.0,
    z: 16.0,
  };
  draw_cube_v(tower_pos, TOWER_SIZE, TOWER_COLOR);
  draw_cube_wires_v(tower_pos, TOWER_SIZE, colors::DARKBLUE);

  tower_pos.x *= -1.0;
  draw_cube_v(tower_pos, TOWER_SIZE, TOWER_COLOR);
  draw_cube_wires_v(tower_pos, TOWER_SIZE, colors::DARKBLUE);

  tower_pos.z *= -1.0;
  draw_cube_v(tower_pos, TOWER_SIZE, TOWER_COLOR);
  draw_cube_wires_v(tower_pos, TOWER_SIZE, colors::DARKBLUE);

  tower_pos.x *= -1.0;
  draw_cube_v(tower_pos, TOWER_SIZE, TOWER_COLOR);
  draw_cube_wires_v(tower_pos, TOWER_SIZE, colors::DARKBLUE);

  draw_sphere(
    Vector3 {
      x: 300.0,
      y: 300.0,
      z: 0.0,
    },
    100.0,
    Color {
      red: 255,
      green: 0,
      blue: 0,
      alpha: 255,
    },
  );
}
