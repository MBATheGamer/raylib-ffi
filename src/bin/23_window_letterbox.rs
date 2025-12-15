fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::WindowResizable, ConfigFlags::VsyncHint]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - window letterbox",
  );
  set_window_min_size(320, 240);

  let game_screen_width = 640;
  let game_screen_height = 480;

  let target = load_render_texture(game_screen_width, game_screen_height);
  set_texture_filter(target.texture, TextureFilter::Bilinear);

  let colors: Vec<Color> = vec![];
  for _ in 0..10 {
    colors.push(Color {
      red: get_random_value(100, 250) as u8,
      green: get_random_value(50, 150) as u8,
      blue: get_random_value(10, 100) as u8,
      alpha: 255,
    });
  }
  set_target_fps(60);

  while !window_should_close() {
    let scale = (get_screen_width() as f32 / game_screen_width as f32)
      .min(get_screen_height() as f32 / game_screen_height as f32);

    if is_key_pressed(KeyboardKey::KeySpace) {
      for i in 0..10 {
        colors[i] = Color {
          red: get_random_value(100, 250) as u8,
          green: get_random_value(50, 150) as u8,
          blue: get_random_value(10, 100) as u8,
          alpha: 255,
        };
      }
    }

    let mouse = get_mouse_position();
    let virtual_mouse = Vector2 {
      x: (mouse.x - (get_screen_width() as f32 - game_screen_width as f32 * scale) * 0.5)
        / scale as f32,
      y: (mouse.y - (get_screen_height() as f32 - game_screen_height as f32 * scale) * 0.5)
        / scale as f32,
    };
    virtual_mouse = virtual_mouse.clamp(
      Vector2::default(),
      Vector2 {
        x: game_screen_width as f32,
        y: game_screen_height as f32,
      },
    );

    begin_texture_mode(target);
    clear_background(colors::RAYWHITE);

    for i in 0..10 {
      draw_rectangle(
        0,
        (game_screen_height / 10) * i,
        game_screen_width,
        game_screen_height / 10,
        colors[i as usize],
      );
    }

    draw_text(
      "If executed inside a window,\nyou can resize the window,\nand see the screen scaling!",
      10,
      25,
      20,
      colors::WHITE,
    );
    draw_text(
      &format!("Default Mouse: [{} , {}]", mouse.x as i32, mouse.y as i32),
      350,
      25,
      20,
      colors::GREEN,
    );
    draw_text(
      &format!(
        "Virtual Mouse: [{} , {}]",
        virtual_mouse.x as i32, virtual_mouse.y as i32
      ),
      350,
      55,
      20,
      colors::YELLOW,
    );
    end_texture_mode();

    begin_drawing();
    clear_background(colors::BLACK);

    draw_texture_pro(
      target.texture,
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: target.texture.width as f32,
        height: -target.texture.height as f32,
      },
      Rectangle {
        x: (get_screen_width() as f32 - game_screen_width as f32 * scale) as f32 * 0.5,
        y: (get_screen_height() as f32 - game_screen_height as f32 * scale) as f32 * 0.5,
        width: (game_screen_width as f32 * scale) as f32,
        height: (game_screen_height as f32 * scale) as f32,
      },
      Vector2 { x: 0.0, y: 0.0 },
      0.0,
      colors::WHITE,
    );
    end_drawing();
  }

  unload_render_texture(target);

  close_window();
}
