use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_2d, begin_texture_mode, clear_background, close_window, end_drawing,
    end_mode_2d, end_texture_mode, get_screen_height, get_screen_width, init_window,
    keyboard::is_key_down, set_target_fps, window_should_close,
  },
  enums::KeyboardKey,
  shape::{draw_line_v, draw_rectangle, draw_rectangle_rec},
  structs::{Camera2D, Rectangle, Vector2},
  text::draw_text,
  texture::{draw_texture_rec, fade, load_render_texture, unload_render_texture},
};

fn main() {
  const PLAYER_SIZE: i32 = 40;

  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 440;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - 2d camera split screen",
  );

  let mut player1 = Rectangle {
    x: 200.0,
    y: 200.0,
    width: PLAYER_SIZE as f32,
    height: PLAYER_SIZE as f32,
  };
  let mut player2 = Rectangle {
    x: 250.0,
    y: 200.0,
    width: PLAYER_SIZE as f32,
    height: PLAYER_SIZE as f32,
  };

  let mut camera1 = Camera2D {
    target: Vector2 {
      x: player1.x,
      y: player1.y,
    },
    offset: Vector2 { x: 200.0, y: 200.0 },
    rotation: 0.0,
    zoom: 1.0,
  };

  let mut camera2 = Camera2D {
    target: Vector2 {
      x: player2.x,
      y: player2.y,
    },
    offset: Vector2 { x: 200.0, y: 200.0 },
    rotation: 0.0,
    zoom: 1.0,
  };

  let screen_camera1 = load_render_texture(SCREEN_WIDTH / 2, SCREEN_HEIGHT);
  let screen_camera2 = load_render_texture(SCREEN_WIDTH / 2, SCREEN_HEIGHT);

  let split_screen_rect = Rectangle {
    x: 0.0,
    y: 0.0,
    width: screen_camera1.texture.width as f32,
    height: -screen_camera1.texture.height as f32,
  };

  set_target_fps(60);

  while !window_should_close() {
    if is_key_down(KeyboardKey::KeyS) {
      player1.y += 3.0;
    } else if is_key_down(KeyboardKey::KeyW) {
      player1.y -= 3.0;
    }
    if is_key_down(KeyboardKey::KeyD) {
      player1.x += 3.0;
    } else if is_key_down(KeyboardKey::KeyA) {
      player1.x -= 3.0;
    }

    if is_key_down(KeyboardKey::KeyUp) {
      player2.y -= 3.0;
    } else if is_key_down(KeyboardKey::KeyDown) {
      player2.y += 3.0;
    }
    if is_key_down(KeyboardKey::KeyRight) {
      player2.x += 3.0;
    } else if is_key_down(KeyboardKey::KeyLeft) {
      player2.x -= 3.0;
    }

    camera1.target = Vector2 {
      x: player1.x,
      y: player1.y,
    };
    camera2.target = Vector2 {
      x: player2.x,
      y: player2.y,
    };

    begin_texture_mode(screen_camera1);
    clear_background(colors::RAYWHITE);

    begin_mode_2d(camera1);

    for i in 0..(SCREEN_WIDTH / PLAYER_SIZE + 1) {
      draw_line_v(
        Vector2 {
          x: (PLAYER_SIZE * i) as f32,
          y: 0.0,
        },
        Vector2 {
          x: (PLAYER_SIZE * i) as f32,
          y: SCREEN_HEIGHT as f32,
        },
        colors::LIGHTGRAY,
      );
    }

    for i in 0..(SCREEN_HEIGHT / PLAYER_SIZE + 1) {
      draw_line_v(
        Vector2 {
          x: 0.0,
          y: (PLAYER_SIZE * i) as f32,
        },
        Vector2 {
          x: SCREEN_WIDTH as f32,
          y: (PLAYER_SIZE * i) as f32,
        },
        colors::LIGHTGRAY,
      );
    }

    for i in 0..(SCREEN_WIDTH / PLAYER_SIZE) {
      for j in 0..(SCREEN_HEIGHT / PLAYER_SIZE) {
        draw_text(
          &format!("[{},{}]", i, j),
          10 + PLAYER_SIZE * i,
          15 + PLAYER_SIZE * j,
          10,
          colors::LIGHTGRAY,
        );
      }
    }

    draw_rectangle_rec(player1, colors::RED);
    draw_rectangle_rec(player2, colors::BLUE);
    end_mode_2d();

    draw_rectangle(
      0,
      0,
      get_screen_width() / 2,
      30,
      fade(colors::RAYWHITE, 0.6),
    );
    draw_text("PLAYER1: W/S/A/D to move", 10, 10, 10, colors::MAROON);

    end_texture_mode();

    begin_texture_mode(screen_camera2);
    clear_background(colors::RAYWHITE);

    begin_mode_2d(camera2);

    for i in 0..(SCREEN_WIDTH / PLAYER_SIZE + 1) {
      draw_line_v(
        Vector2 {
          x: (PLAYER_SIZE * i) as f32,
          y: 0.0,
        },
        Vector2 {
          x: (PLAYER_SIZE * i) as f32,
          y: SCREEN_HEIGHT as f32,
        },
        colors::LIGHTGRAY,
      );
    }

    for i in 0..(SCREEN_HEIGHT / PLAYER_SIZE + 1) {
      draw_line_v(
        Vector2 {
          x: 0.0,
          y: (PLAYER_SIZE * i) as f32,
        },
        Vector2 {
          x: SCREEN_WIDTH as f32,
          y: (PLAYER_SIZE * i) as f32,
        },
        colors::LIGHTGRAY,
      );
    }

    for i in 0..(SCREEN_WIDTH / PLAYER_SIZE) {
      for j in 0..(SCREEN_HEIGHT / PLAYER_SIZE) {
        draw_text(
          &format!("[{},{}]", i, j),
          10 + PLAYER_SIZE * i,
          15 + PLAYER_SIZE * j,
          10,
          colors::LIGHTGRAY,
        );
      }
    }

    draw_rectangle_rec(player1, colors::RED);
    draw_rectangle_rec(player2, colors::BLUE);

    end_mode_2d();

    draw_rectangle(
      0,
      0,
      get_screen_width() / 2,
      30,
      fade(colors::RAYWHITE, 0.6),
    );
    draw_text(
      "PLAYER2: UP/DOWN/LEFT/RIGHT to move",
      10,
      10,
      10,
      colors::DARKBLUE,
    );

    end_texture_mode();

    begin_drawing();
    clear_background(colors::BLACK);

    draw_texture_rec(
      screen_camera1.texture,
      split_screen_rect,
      Vector2 { x: 0.0, y: 0.0 },
      colors::WHITE,
    );
    draw_texture_rec(
      screen_camera2.texture,
      split_screen_rect,
      Vector2 {
        x: (SCREEN_WIDTH / 2) as f32,
        y: 0.0,
      },
      colors::WHITE,
    );

    draw_rectangle(
      get_screen_width() / 2 - 2,
      0,
      4,
      get_screen_height(),
      colors::LIGHTGRAY,
    );
    end_drawing();
  }

  unload_render_texture(screen_camera1);
  unload_render_texture(screen_camera2);

  close_window();
}
