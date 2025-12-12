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

static SENSITIVITY: Vector2 = Vector2 { x: 0.001, y: 0.001 };

static PLAYER: Body = Body {
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
};
static LOOK_ROTATION: Vector2 = Vector2 { x: 0.0, y: 0.0 };
static HEAD_TIMER: f32 = 0.0;
static WALK_LERP: f32 = 0.0;
static HEAD_LERP: f32 = STAND_HEIGHT;
static LEAN: Vector2 = Vector2 { x: 0.0, y: 0.0 };

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 3d camera fps",
  );

  let camera = Camera3D {
    fovy: 60.0,
    projection: CameraProjection::Perspective,
    position: Vector3 {
      x: PLAYER.position.x,
      y: PLAYER.position.y + (BOTTOM_HEIGHT + HEAD_LERP),
      z: PLAYER.position.z,
    },
    target: Vector3::default(),
    up: Vector3::default(),
  };

  update_camera_fps(&camera);

  disable_cursor();

  set_target_fps(60);

  while !window_should_close() {
    let mouse_delta = get_mouse_delta();
    LOOK_ROTATION.x -= mouse_delta.x * SENSITIVITY.x;
    LOOK_ROTATION.y += mouse_delta.y * SENSITIVITY.y;

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
      &PLAYER,
      LOOK_ROTATION.x,
      sideway,
      forward,
      is_key_pressed(KeyboardKey::KeySpace),
      crouching,
    );

    let delta = get_frame_time();
    HEAD_LERP = HEAD_LERP.lerp(
      if crouching {
        CROUCH_HEIGHT
      } else {
        STAND_HEIGHT
      },
      20.0 * delta,
    );
    camera.position = Vector3 {
      x: PLAYER.position.x,
      y: PLAYER.position.y + (BOTTOM_HEIGHT + HEAD_LERP),
      z: PLAYER.position.z,
    };

    if PLAYER.is_grounded && (forward != 0.0 || sideway != 0.0) {
      HEAD_TIMER += delta * 3.0;
      WALK_LERP = WALK_LERP.lerp(1.0, 10.0 * delta);
      camera.fovy = camera.fovy.lerp(55.0, 5.0 * delta);
    } else {
      WALK_LERP = WALK_LERP.lerp(0.0, 10.0 * delta);
      camera.fovy = camera.fovy.lerp(60.0, 5.0 * delta);
    }

    LEAN.x = LEAN.x.lerp(sideway * 0.02, 10.0 * delta);
    LEAN.y = LEAN.y.lerp(forward * 0.015, 10.0 * delta);

    update_camera_fps(&camera);

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
          x: PLAYER.velocity.x,
          y: PLAYER.velocity.z
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
  body: &Body,
  rot: f32,
  side: f32,
  forward: f32,
  jump_pressed: bool,
  crouch_hold: bool,
) {
  let input = Vector2 {
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
    x: -rot.cos(),
    y: 0.0,
    z: -rot.sin(),
  };

  let desired_dir = Vector3 {
    x: input.x * right.x + input.y * front.x,
    y: 0.0,
    z: input.x * right.z + input.y * front.z,
  };
  body.dir = body.dir.lerp(desired_dir, CONTROL * delta);

  let decel = if body.is_grounded { FRICTION } else { AIR_DRAG };
  let hvel = Vector3 {
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

fn update_camera_fps(camera: &Camera3D) {
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

  let yaw = TARGET_OFFSET.rotate_by_axis_angle(UP, LOOK_ROTATION.x);

  let max_angle_up = UP.angle(yaw);
  max_angle_up -= 0.001;
  if -LOOK_ROTATION.y > max_angle_up {
    LOOK_ROTATION.y = -max_angle_up;
  }

  let max_angle_down = (-UP).angle(yaw);
  max_angle_down *= -1.0;
  max_angle_down += 0.001;
  if -LOOK_ROTATION.y < max_angle_down {
    LOOK_ROTATION.y = -max_angle_down;
  }

  let right = yaw.cross_product(UP).normalize();

  let pitch_angle = -LOOK_ROTATION.y - LEAN.y;
  pitch_angle = pitch_angle.clamp(-PI / 2.0 + 0.0001, PI / 2.0 - 0.0001);

  let pitch = yaw.rotate_by_axis_angle(right, pitch_angle);

  let head_sin = (HEAD_TIMER * PI).sin();
  let head_cos = (HEAD_TIMER * PI).cos();
  const STEP_ROTATION: f32 = 0.01;
  camera.up = UP.rotate_by_axis_angle(pitch, head_sin * STEP_ROTATION + LEAN.x);

  const BOB_SIDE: f32 = 0.1;
  const BOB_UP: f32 = 0.15;
  let bobbing = right.scale(head_sin * BOB_SIDE);
  bobbing.y = (head_cos * BOB_UP).abs();

  camera.position = camera.position + bobbing.scale(WALK_LERP);
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

  let tower_pos = Vector3 {
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
