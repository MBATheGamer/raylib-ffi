use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    export_automation_event_list, get_world_to_screen_2d, init_window, is_file_dropped,
    is_file_extension,
    keyboard::{is_key_down, is_key_pressed},
    load_automation_event_list, load_dropped_files,
    mouse::get_mouse_wheel_move,
    play_automation_event, set_automation_event_base_frame, set_automation_event_list,
    set_target_fps, start_automation_event_recording, stop_automation_event_recording,
    unload_automation_event_list, unload_dropped_files, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_circle, draw_rectangle, draw_rectangle_lines, draw_rectangle_rec, draw_triangle},
  structs::{Camera2D, Color, Rectangle, Vector2},
  text::draw_text,
  texture::fade,
};

const GRAVITY: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

const MAX_ENVIRONMENT_ELEMENTS: usize = 5;

struct Player {
  position: Vector2,
  speed: f32,
  can_jump: bool,
}

struct EnvElement {
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
    "raylib [core] example - automation events",
  );

  let mut player = Player {
    position: Vector2 { x: 400.0, y: 280.0 },
    speed: 0.0,
    can_jump: false,
  };

  let env_elements: Vec<EnvElement> = vec![
    EnvElement {
      rect: Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 400.0,
      },
      blocking: false,
      color: colors::LIGHTGRAY,
    },
    EnvElement {
      rect: Rectangle {
        x: 0.0,
        y: 400.0,
        width: 1000.0,
        height: 200.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvElement {
      rect: Rectangle {
        x: 300.0,
        y: 200.0,
        width: 400.0,
        height: 10.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvElement {
      rect: Rectangle {
        x: 250.0,
        y: 300.0,
        width: 100.0,
        height: 10.0,
      },
      blocking: true,
      color: colors::GRAY,
    },
    EnvElement {
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

  let mut automation_event_list = load_automation_event_list("");
  set_automation_event_list(&mut automation_event_list);
  let mut event_recording = false;
  let mut event_playing = false;

  let mut frame_counter: u32 = 0;
  let mut play_frame_counter: u32 = 0;
  let mut current_play_frame: u32 = 0;

  set_target_fps(60);

  while !window_should_close() {
    let delta_time = 0.015;

    if is_file_dropped() {
      let dropped_files = load_dropped_files();

      if let Some(dropped_file) = dropped_files.get(0)
        && is_file_extension(dropped_file, ".txt;.rae")
      {
        unload_automation_event_list(automation_event_list);
        automation_event_list = load_automation_event_list(dropped_file);

        event_recording = false;

        event_playing = true;
        play_frame_counter = 0;
        current_play_frame = 0;

        player.position = Vector2 { x: 400.0, y: 280.0 };
        player.speed = 0.0;
        player.can_jump = false;

        camera.target = player.position;
        camera.offset = Vector2 {
          x: SCREEN_WIDTH as f32 / 2.0,
          y: SCREEN_HEIGHT as f32 / 2.0,
        };
        camera.rotation = 0.0;
        camera.zoom = 1.0;
      }

      unload_dropped_files(dropped_files);
    }

    if is_key_down(KeyboardKey::KeyLeft) {
      player.position.x -= PLAYER_HOR_SPD * delta_time;
    }
    if is_key_down(KeyboardKey::KeyRight) {
      player.position.x += PLAYER_HOR_SPD * delta_time;
    }
    if is_key_down(KeyboardKey::KeySpace) && player.can_jump {
      player.speed = -PLAYER_JUMP_SPD;
      player.can_jump = false;
    }

    let mut hit_obstacle = false;
    for i in 0..MAX_ENVIRONMENT_ELEMENTS {
      let element = &env_elements[i];
      let p = &mut player.position;
      if element.blocking
        && element.rect.x <= p.x
        && element.rect.x + element.rect.width >= p.x
        && element.rect.y >= p.y
        && element.rect.y <= p.y + player.speed * delta_time
      {
        hit_obstacle = true;
        player.speed = 0.0;
        p.y = element.rect.y;
      }
    }

    if !hit_obstacle {
      player.position.y += player.speed * delta_time;
      player.speed += GRAVITY * delta_time;
      player.can_jump = false;
    } else {
      player.can_jump = true;
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      player.position = Vector2 { x: 400.0, y: 280.0 };
      player.speed = 0.0;
      player.can_jump = false;

      camera.target = player.position;
      camera.offset = Vector2 {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0,
      };
      camera.rotation = 0.0;
      camera.zoom = 1.0;
    }

    if event_playing {
      let mut automation_event = automation_event_list
        .get(current_play_frame as usize)
        .unwrap();
      while play_frame_counter == automation_event.frame {
        play_automation_event(*automation_event);
        current_play_frame += 1;

        if current_play_frame == automation_event_list.count {
          event_playing = false;
          current_play_frame = 0;
          play_frame_counter = 0;

          // trace_log(LOG_INFO, "FINISH PLAYING!");
          println!("[INFO] FINISH PLAYING!");
          break;
        }

        automation_event = automation_event_list
          .get(current_play_frame as usize)
          .unwrap();
      }

      play_frame_counter += 1;
    }

    camera.target = player.position;
    camera.offset = Vector2 {
      x: SCREEN_WIDTH as f32 / 2.0,
      y: SCREEN_HEIGHT as f32 / 2.0,
    };
    let mut min_x = 1000.0;
    let mut min_y = 1000.0;
    let mut max_x = -1000.0;
    let mut max_y = -1000.0;

    camera.zoom += get_mouse_wheel_move() * 0.05;
    if camera.zoom > 3.0 {
      camera.zoom = 3.0;
    } else if camera.zoom < 0.25 {
      camera.zoom = 0.25;
    }

    for i in 0..MAX_ENVIRONMENT_ELEMENTS {
      let element = &env_elements[i];
      min_x = element.rect.x.min(min_x);
      max_x = (element.rect.x + element.rect.width).max(max_x);
      min_y = element.rect.y.min(min_y);
      max_y = (element.rect.y + element.rect.height).max(max_y);
    }

    let max = get_world_to_screen_2d(Vector2 { x: max_x, y: max_y }, camera);
    let min = get_world_to_screen_2d(Vector2 { x: min_x, y: min_y }, camera);

    if max.x < SCREEN_WIDTH as f32 {
      camera.offset.x = SCREEN_WIDTH as f32 - (max.x - SCREEN_WIDTH as f32 / 2.0);
    }
    if max.y < SCREEN_HEIGHT as f32 {
      camera.offset.y = SCREEN_HEIGHT as f32 - (max.y - SCREEN_HEIGHT as f32 / 2.0);
    }
    if min.x > 0.0 {
      camera.offset.x = SCREEN_WIDTH as f32 / 2.0 - min.x;
    }
    if min.y > 0.0 {
      camera.offset.y = SCREEN_HEIGHT as f32 / 2.0 - min.y;
    }

    if is_key_pressed(KeyboardKey::KeyS) {
      if !event_playing {
        if event_recording {
          stop_automation_event_recording();
          event_recording = false;

          export_automation_event_list(automation_event_list, "automation.rae");

          // trace_log(LOG_INFO, "[INFO] RECORDED FRAMES: %i", automation_event_list.count);
          println!("[INFO] RECORDED FRAMES: {}", automation_event_list.count);
        } else {
          set_automation_event_base_frame(180);
          start_automation_event_recording();
          event_recording = true;
        }
      }
    } else if is_key_pressed(KeyboardKey::KeyA) {
      if !event_recording && automation_event_list.count > 0 {
        event_playing = true;
        play_frame_counter = 0;
        current_play_frame = 0;

        player.position = Vector2 { x: 400.0, y: 280.0 };
        player.speed = 0.0;
        player.can_jump = false;

        camera.target = player.position;
        camera.offset = Vector2 {
          x: SCREEN_WIDTH as f32 / 2.0,
          y: SCREEN_HEIGHT as f32 / 2.0,
        };
        camera.rotation = 0.0;
        camera.zoom = 1.0;
      }
    }

    if event_recording || event_playing {
      frame_counter += 1;
    } else {
      frame_counter = 0;
    }

    begin_drawing();

    clear_background(colors::LIGHTGRAY);

    begin_mode_2d(camera);

    for i in 0..MAX_ENVIRONMENT_ELEMENTS {
      draw_rectangle_rec(env_elements[i].rect, env_elements[i].color);
    }

    draw_rectangle_rec(
      Rectangle {
        x: player.position.x - 20.0,
        y: player.position.y - 40.0,
        width: 40.0,
        height: 40.0,
      },
      colors::RED,
    );

    end_mode_2d();

    draw_rectangle(10, 10, 290, 145, fade(colors::SKYBLUE, 0.5));
    draw_rectangle_lines(10, 10, 290, 145, fade(colors::BLUE, 0.8));

    draw_text("Controls:", 20, 20, 10, colors::BLACK);
    draw_text(
      "- RIGHT | LEFT: Player movement",
      30,
      40,
      10,
      colors::DARKGRAY,
    );
    draw_text("- SPACE: Player jump", 30, 60, 10, colors::DARKGRAY);
    draw_text("- R: Reset game state", 30, 80, 10, colors::DARKGRAY);

    draw_text(
      "- S: START/STOP RECORDING INPUT EVENTS",
      30,
      110,
      10,
      colors::BLACK,
    );
    draw_text(
      "- A: REPLAY LAST RECORDED INPUT EVENTS",
      30,
      130,
      10,
      colors::BLACK,
    );

    if event_recording {
      draw_rectangle(10, 160, 290, 30, fade(colors::RED, 0.3));
      draw_rectangle_lines(10, 160, 290, 30, fade(colors::MAROON, 0.8));
      draw_circle(30, 175, 10.0, colors::MAROON);

      if (frame_counter / 15) % 2 == 1 {
        draw_text(
          &format!("RECORDING EVENTS... [{}]", automation_event_list.count),
          50,
          170,
          10,
          colors::MAROON,
        );
      }
    } else if event_playing {
      draw_rectangle(10, 160, 290, 30, fade(colors::LIME, 0.3));
      draw_rectangle_lines(10, 160, 290, 30, fade(colors::DARKGREEN, 0.8));
      draw_triangle(
        Vector2 {
          x: 20.0,
          y: 155.0 + 10.0,
        },
        Vector2 {
          x: 20.0,
          y: 155.0 + 30.0,
        },
        Vector2 {
          x: 40.0,
          y: 155.0 + 20.0,
        },
        colors::DARKGREEN,
      );

      if (frame_counter / 15) % 2 == 1 {
        draw_text(
          &format!("PLAYING RECORDED EVENTS... [{}]", current_play_frame),
          50,
          170,
          10,
          colors::DARKGREEN,
        );
      }
    }

    end_drawing();
  }

  close_window();
}
