use std::sync::{LazyLock, Mutex};

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    get_frame_time, get_screen_to_world_2d, get_world_to_screen_2d, init_window,
    keyboard::{is_key_down, is_key_pressed},
    mouse::get_mouse_wheel_move,
    set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_circle_v, draw_rectangle_rec},
  structs::{Camera2D, Color, Rectangle, Vector2},
  text::draw_text,
};

const G: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

struct Player {
  position: Vector2,
  speed: f32,
  can_jump: bool,
}

struct EnvItem {
  rect: Rectangle,
  blocking: bool,
  color: Color,
}

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 2d camera platformer",
  );

  let mut player = Player {
    position: Vector2 { x: 400.0, y: 280.0 },
    speed: 0.0,
    can_jump: false,
  };
  let env_items: Vec<EnvItem> = vec![
    EnvItem {
      rect: Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 400.0,
      },
      blocking: false,
      color: colors::LIGHTGRAY,
    },
    EnvItem {
      rect: Rectangle {
        x: 0.0,
        y: 400.0,
        width: 1000.0,
        height: 200.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvItem {
      rect: Rectangle {
        x: 300.0,
        y: 200.0,
        width: 400.0,
        height: 10.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvItem {
      rect: Rectangle {
        x: 250.0,
        y: 300.0,
        width: 100.0,
        height: 10.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvItem {
      rect: Rectangle {
        x: 650.0,
        y: 300.0,
        width: 100.0,
        height: 10.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
  ];

  let mut camera = Camera2D {
    target: player.position,
    offset: Vector2 {
      x: SCREEN_WIDTH as f32 / 2.0,
      y: SCREEN_HEIGHT as f32 / 2.0,
    },
    rotation: 0.0,
    zoom: 1.0,
  };

  let camera_updaters: Vec<fn(&mut Camera2D, &mut Player, &Vec<EnvItem>, f32, i32, i32)> = vec![
    update_camera_center,
    update_camera_center_inside_map,
    update_camera_center_smooth_follow,
    update_camera_even_out_on_landing,
    update_camera_player_bounds_push,
  ];

  let mut camera_option = 0;

  let camera_descriptions: Vec<&str> = vec![
    "Follow player center",
    "Follow player center, but clamp to map edges",
    "Follow player center; smoothed",
    "Follow player center horizontally; update player center vertically after landing",
    "Player push camera on getting too close to screen edge",
  ];

  set_target_fps(60);

  while !window_should_close() {
    let delta_time = get_frame_time();

    update_player(&mut player, &env_items, delta_time);

    camera.zoom += get_mouse_wheel_move() as f32 * 0.05;

    if camera.zoom > 3.0 {
      camera.zoom = 3.0;
    } else if camera.zoom < 0.25 {
      camera.zoom = 0.25;
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      camera.zoom = 1.0;
      player.position = Vector2 { x: 400.0, y: 280.0 };
    }

    if is_key_pressed(KeyboardKey::KeyC) {
      camera_option = (camera_option + 1) % camera_updaters.len();
    }

    camera_updaters[camera_option](
      &mut camera,
      &mut player,
      &env_items,
      delta_time,
      SCREEN_WIDTH,
      SCREEN_HEIGHT,
    );

    begin_drawing();

    clear_background(colors::LIGHTGRAY);

    begin_mode_2d(camera);

    for i in 0..env_items.len() {
      draw_rectangle_rec(env_items[i].rect, env_items[i].color);
    }

    let player_rect = Rectangle {
      x: player.position.x - 20.0,
      y: player.position.y - 40.0,
      width: 40.0,
      height: 40.0,
    };
    draw_rectangle_rec(player_rect, colors::RED);

    draw_circle_v(player.position, 5.0, colors::GOLD);

    end_mode_2d();

    draw_text("Controls:", 20, 20, 10, colors::BLACK);
    draw_text("- Right/Left to move", 40, 40, 10, colors::DARKGRAY);
    draw_text("- Space to jump", 40, 60, 10, colors::DARKGRAY);
    draw_text(
      "- Mouse Wheel to Zoom in-out, R to reset zoom",
      40,
      80,
      10,
      colors::DARKGRAY,
    );
    draw_text("- C to change camera mode", 40, 100, 10, colors::DARKGRAY);
    draw_text("Current camera mode:", 20, 120, 10, colors::BLACK);
    draw_text(
      camera_descriptions[camera_option],
      40,
      140,
      10,
      colors::DARKGRAY,
    );

    end_drawing();
  }

  close_window();
}

fn update_player(player: &mut Player, env_items: &Vec<EnvItem>, delta: f32) {
  if is_key_down(KeyboardKey::KeyLeft) {
    player.position.x -= PLAYER_HOR_SPD * delta;
  }
  if is_key_down(KeyboardKey::KeyRight) {
    player.position.x += PLAYER_HOR_SPD * delta;
  }
  if is_key_down(KeyboardKey::KeySpace) && player.can_jump {
    player.speed = -PLAYER_JUMP_SPD;
    player.can_jump = false;
  }

  let mut hit_obstacle = false;
  for i in 0..env_items.len() {
    let ei = &env_items[i];
    let mut p = player.position;
    if ei.blocking
      && ei.rect.x <= p.x
      && ei.rect.x + ei.rect.width >= p.x
      && ei.rect.y >= p.y
      && ei.rect.y <= p.y + player.speed * delta
    {
      hit_obstacle = true;
      player.speed = 0.0;
      p.y = ei.rect.y;
      break;
    }
  }

  if !hit_obstacle {
    player.position.y += player.speed * delta;
    player.speed += G * delta;
    player.can_jump = false;
  } else {
    player.can_jump = true;
  }
}

fn update_camera_center(
  camera: &mut Camera2D,
  player: &mut Player,
  _: &Vec<EnvItem>,
  _: f32,
  width: i32,
  height: i32,
) {
  camera.offset = Vector2 {
    x: width as f32 / 2.0,
    y: height as f32 / 2.0,
  };
  camera.target = player.position;
}

fn update_camera_center_inside_map(
  camera: &mut Camera2D,
  player: &mut Player,
  env_items: &Vec<EnvItem>,
  _: f32,
  width: i32,
  height: i32,
) {
  camera.target = player.position;
  camera.offset = Vector2 {
    x: width as f32 / 2.0,
    y: height as f32 / 2.0,
  };
  let mut min_x = 1000.0;
  let mut min_y = 1000.0;
  let mut max_x = -1000.0;
  let mut max_y = -1000.0;

  for i in 0..env_items.len() {
    let ei = &env_items[i];
    min_x = ei.rect.x.min(min_x);
    max_x = (ei.rect.x + ei.rect.width).max(max_x);
    min_y = ei.rect.y.min(min_y);
    max_y = (ei.rect.y + ei.rect.height).max(max_y);
  }

  let max = get_world_to_screen_2d(Vector2 { x: max_x, y: max_y }, *camera);
  let min = get_world_to_screen_2d(Vector2 { x: min_x, y: min_y }, *camera);

  if max.x < width as f32 {
    camera.offset.x = width as f32 - (max.x - width as f32 / 2.0) as f32;
  }
  if max.y < height as f32 {
    camera.offset.y = height as f32 - (max.y - height as f32 / 2.0);
  }
  if min.x > 0.0 {
    camera.offset.x = width as f32 / 2.0 - min.x;
  }
  if min.y > 0.0 {
    camera.offset.y = height as f32 / 2.0 - min.y;
  }
}

fn update_camera_center_smooth_follow(
  camera: &mut Camera2D,
  player: &mut Player,
  _: &Vec<EnvItem>,
  delta: f32,
  width: i32,
  height: i32,
) {
  static MIN_SPEED: f32 = 30.0;
  static MIN_EFFECT_LENGTH: f32 = 10.0;
  static FRACTION_SPEED: f32 = 0.8;

  camera.offset = Vector2 {
    x: width as f32 / 2.0,
    y: height as f32 / 2.0,
  };
  let diff = player.position - camera.target;
  let length = diff.length();

  if length > MIN_EFFECT_LENGTH {
    let speed = (FRACTION_SPEED * length).max(MIN_SPEED);
    camera.target = camera.target + diff * (speed * delta / length);
  }
}

fn update_camera_even_out_on_landing(
  camera: &mut Camera2D,
  player: &mut Player,
  _: &Vec<EnvItem>,
  delta: f32,
  width: i32,
  height: i32,
) {
  static EVEN_OUT_SPEED: f32 = 700.0;
  static EVENING_OUT: LazyLock<Mutex<bool>> = LazyLock::new(|| {
    return Mutex::new(false);
  });
  static EVEN_OUT_TARGET: LazyLock<Mutex<f32>> = LazyLock::new(|| {
    return Mutex::new(0.0);
  });

  let mut evening_out = EVENING_OUT
    .lock()
    .expect("Expecting EVENING_OUT has a value");
  let mut evening_out_target = EVEN_OUT_TARGET
    .lock()
    .expect("Expecting EVEN_OUT_TARGET has a value");

  camera.offset = Vector2 {
    x: width as f32 / 2.0,
    y: height as f32 / 2.0,
  };
  camera.target.x = player.position.x;

  if *evening_out {
    if *evening_out_target > camera.target.y {
      camera.target.y += EVEN_OUT_SPEED * delta;

      if camera.target.y > *evening_out_target {
        camera.target.y = *evening_out_target;
        *evening_out = false;
      }
    } else {
      camera.target.y -= EVEN_OUT_SPEED * delta;

      if camera.target.y < *evening_out_target {
        camera.target.y = *evening_out_target;
        *evening_out = false;
      }
    }
  } else {
    if player.can_jump && player.speed == 0.0 && player.position.y != camera.target.y {
      *evening_out = true;
      *evening_out_target = player.position.y;
    }
  }
}

fn update_camera_player_bounds_push(
  camera: &mut Camera2D,
  player: &mut Player,
  _: &Vec<EnvItem>,
  _: f32,
  width: i32,
  height: i32,
) {
  static BBOX: Vector2 = Vector2 { x: 0.2, y: 0.2 };

  let bbox_world_min = get_screen_to_world_2d(
    Vector2 {
      x: (1.0 - BBOX.x) * 0.5 * width as f32,
      y: (1.0 - BBOX.y) * 0.5 * height as f32,
    },
    *camera,
  );
  let bbox_world_max = get_screen_to_world_2d(
    Vector2 {
      x: (1.0 + BBOX.x) * 0.5 * width as f32,
      y: (1.0 + BBOX.y) * 0.5 * height as f32,
    },
    *camera,
  );
  camera.offset = Vector2 {
    x: (1.0 - BBOX.x) * 0.5 * width as f32,
    y: (1.0 - BBOX.y) * 0.5 * height as f32,
  };

  if player.position.x < bbox_world_min.x {
    camera.target.x = player.position.x;
  }
  if player.position.y < bbox_world_min.y {
    camera.target.y = player.position.y;
  }
  if player.position.x > bbox_world_max.x {
    camera.target.x = bbox_world_min.x + (player.position.x - bbox_world_max.x);
  }
  if player.position.y > bbox_world_max.y {
    camera.target.y = bbox_world_min.y + (player.position.y - bbox_world_max.y);
  }
}
