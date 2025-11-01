use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, clear_window_state, close_window, end_drawing,
    get_screen_height, get_screen_width, init_window, is_window_state, keyboard::is_key_pressed,
    maximize_window, minimize_window, mouse::get_mouse_position, restore_window, set_target_fps,
    set_window_state, toggle_borderless_windowed, toggle_fullscreen, window_should_close,
  },
  enums::{ConfigFlags, KeyboardKey},
  shape::{draw_circle_v, draw_rectangle_lines_ex},
  structs::{Rectangle, Vector2},
  text::{draw_fps, draw_text},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - window flags",
  );

  let mut ball_position = Vector2 {
    x: get_screen_width() as f32 / 2.0,
    y: get_screen_height() as f32 / 2.0,
  };
  let mut ball_speed = Vector2 { x: 5.0, y: 4.0 };
  let ball_radius = 20.0;

  let mut frames_counter = 0;

  set_target_fps(60);

  while !window_should_close() {
    if is_key_pressed(KeyboardKey::KeyF) {
      toggle_fullscreen();
    }

    if is_key_pressed(KeyboardKey::KeyR) {
      if is_window_state(ConfigFlags::WindowResizable) {
        clear_window_state(ConfigFlags::WindowResizable);
      } else {
        set_window_state(ConfigFlags::WindowResizable);
      }
    }

    if is_key_pressed(KeyboardKey::KeyD) {
      if is_window_state(ConfigFlags::WindowUndecorated) {
        clear_window_state(ConfigFlags::WindowUndecorated);
      } else {
        set_window_state(ConfigFlags::WindowUndecorated);
      }
    }

    if is_key_pressed(KeyboardKey::KeyH) {
      if !is_window_state(ConfigFlags::WindowHidden) {
        set_window_state(ConfigFlags::WindowHidden);
      }

      frames_counter = 0;
    }

    if is_window_state(ConfigFlags::WindowHidden) {
      frames_counter += 1;
      if frames_counter >= 240 {
        clear_window_state(ConfigFlags::WindowHidden);
      }
    }

    if is_key_pressed(KeyboardKey::KeyN) {
      if !is_window_state(ConfigFlags::WindowMinimized) {
        minimize_window();
      }

      frames_counter = 0;
    }

    if is_window_state(ConfigFlags::WindowMinimized) {
      frames_counter += 1;
      if frames_counter >= 240 {
        restore_window();
        frames_counter = 0;
      }
    }

    if is_key_pressed(KeyboardKey::KeyM) {
      if is_window_state(ConfigFlags::WindowMaximized) {
        restore_window();
      } else {
        maximize_window();
      }
    }

    if is_key_pressed(KeyboardKey::KeyU) {
      if is_window_state(ConfigFlags::WindowUnfocused) {
        clear_window_state(ConfigFlags::WindowUnfocused);
      } else {
        set_window_state(ConfigFlags::WindowUnfocused);
      }
    }

    if is_key_pressed(KeyboardKey::KeyT) {
      if is_window_state(ConfigFlags::WindowTopmost) {
        clear_window_state(ConfigFlags::WindowTopmost);
      } else {
        set_window_state(ConfigFlags::WindowTopmost);
      }
    }

    if is_key_pressed(KeyboardKey::KeyA) {
      if is_window_state(ConfigFlags::WindowAlwaysRun) {
        clear_window_state(ConfigFlags::WindowAlwaysRun);
      } else {
        set_window_state(ConfigFlags::WindowAlwaysRun);
      }
    }

    if is_key_pressed(KeyboardKey::KeyV) {
      if is_window_state(ConfigFlags::VsyncHint) {
        clear_window_state(ConfigFlags::VsyncHint);
      } else {
        set_window_state(ConfigFlags::VsyncHint);
      }
    }

    if is_key_pressed(KeyboardKey::KeyB) {
      toggle_borderless_windowed();
    }

    // Bouncing ball logic
    ball_position.x += ball_speed.x;
    ball_position.y += ball_speed.y;
    if ball_position.x >= (get_screen_width() as f32 - ball_radius)
      || ball_position.x <= ball_radius
    {
      ball_speed.x *= -1.0;
    }
    if ball_position.y >= (get_screen_height() as f32 - ball_radius)
      || ball_position.y <= ball_radius
    {
      ball_speed.y *= -1.0;
    }

    begin_drawing();

    if is_window_state(ConfigFlags::WindowTransparent) {
      clear_background(colors::BLANK);
    } else {
      clear_background(colors::RAYWHITE);
    }

    draw_circle_v(ball_position, ball_radius, colors::MAROON);
    draw_rectangle_lines_ex(
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: get_screen_width() as f32,
        height: get_screen_height() as f32,
      },
      4.0,
      colors::RAYWHITE,
    );

    draw_circle_v(get_mouse_position(), 10.0, colors::DARKBLUE);

    draw_fps(10, 10);

    draw_text(
      &format!(
        "Screen Size: [{}, {}]",
        get_screen_width(),
        get_screen_height(),
      ),
      10,
      40,
      10,
      colors::GREEN,
    );

    draw_text(
      "Following flags can be set after window creation:",
      10,
      60,
      10,
      colors::GRAY,
    );
    if is_window_state(ConfigFlags::FullscreenMode) {
      draw_text("[F] FLAG_FULLSCREEN_MODE: on", 10, 80, 10, colors::LIME);
    } else {
      draw_text("[F] FLAG_FULLSCREEN_MODE: off", 10, 80, 10, colors::MAROON);
    }
    if is_window_state(ConfigFlags::WindowResizable) {
      draw_text("[R] FLAG_WINDOW_RESIZABLE: on", 10, 100, 10, colors::LIME);
    } else {
      draw_text(
        "[R] FLAG_WINDOW_RESIZABLE: off",
        10,
        100,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowUndecorated) {
      draw_text("[D] FLAG_WINDOW_UNDECORATED: on", 10, 120, 10, colors::LIME);
    } else {
      draw_text(
        "[D] FLAG_WINDOW_UNDECORATED: off",
        10,
        120,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowHidden) {
      draw_text("[H] FLAG_WINDOW_HIDDEN: on", 10, 140, 10, colors::LIME);
    } else {
      draw_text(
        "[H] FLAG_WINDOW_HIDDEN: off (hides for 3 seconds)",
        10,
        140,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowMinimized) {
      draw_text("[N] FLAG_WINDOW_MINIMIZED: on", 10, 160, 10, colors::LIME);
    } else {
      draw_text(
        "[N] FLAG_WINDOW_MINIMIZED: off (restores after 3 seconds)",
        10,
        160,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowMaximized) {
      draw_text("[M] FLAG_WINDOW_MAXIMIZED: on", 10, 180, 10, colors::LIME);
    } else {
      draw_text(
        "[M] FLAG_WINDOW_MAXIMIZED: off",
        10,
        180,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowUnfocused) {
      draw_text("[G] FLAG_WINDOW_UNFOCUSED: on", 10, 200, 10, colors::LIME);
    } else {
      draw_text(
        "[U] FLAG_WINDOW_UNFOCUSED: off",
        10,
        200,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::WindowTopmost) {
      draw_text("[T] FLAG_WINDOW_TOPMOST: on", 10, 220, 10, colors::LIME);
    } else {
      draw_text("[T] FLAG_WINDOW_TOPMOST: off", 10, 220, 10, colors::MAROON);
    }
    if is_window_state(ConfigFlags::WindowAlwaysRun) {
      draw_text("[A] FLAG_WINDOW_ALWAYS_RUN: on", 10, 240, 10, colors::LIME);
    } else {
      draw_text(
        "[A] FLAG_WINDOW_ALWAYS_RUN: off",
        10,
        240,
        10,
        colors::MAROON,
      );
    }
    if is_window_state(ConfigFlags::VsyncHint) {
      draw_text("[V] FLAG_VSYNC_HINT: on", 10, 260, 10, colors::LIME);
    } else {
      draw_text("[V] FLAG_VSYNC_HINT: off", 10, 260, 10, colors::MAROON);
    }
    if is_window_state(ConfigFlags::BorderlessWindowedMode) {
      draw_text(
        "[B] FLAG_BORDERLESS_WINDOWED_MODE: on",
        10,
        280,
        10,
        colors::LIME,
      );
    } else {
      draw_text(
        "[B] FLAG_BORDERLESS_WINDOWED_MODE: off",
        10,
        280,
        10,
        colors::MAROON,
      );
    }

    draw_text(
      "Following flags can only be set before window creation:",
      10,
      320,
      10,
      colors::GRAY,
    );
    if is_window_state(ConfigFlags::WindowHighDPI) {
      draw_text("FLAG_WINDOW_HIGHDPI: on", 10, 340, 10, colors::LIME);
    } else {
      draw_text("FLAG_WINDOW_HIGHDPI: off", 10, 340, 10, colors::MAROON);
    }
    if is_window_state(ConfigFlags::WindowTransparent) {
      draw_text("FLAG_WINDOW_TRANSPARENT: on", 10, 360, 10, colors::LIME);
    } else {
      draw_text("FLAG_WINDOW_TRANSPARENT: off", 10, 360, 10, colors::MAROON);
    }
    if is_window_state(ConfigFlags::MSAA4xHint) {
      draw_text("FLAG_MSAA_4X_HINT: on", 10, 380, 10, colors::LIME);
    } else {
      draw_text("FLAG_MSAA_4X_HINT: off", 10, 380, 10, colors::MAROON);
    }

    end_drawing();
  }

  close_window();
}
