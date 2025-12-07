fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - background scrolling",
  );

  let background = load_texture("resources/cyberpunk_street_background.png");
  let midground = load_texture("resources/cyberpunk_street_midground.png");
  let foreground = load_texture("resources/cyberpunk_street_foreground.png");

  let scrolling_back = 0.0;
  let scrolling_mid = 0.0;
  let scrolling_fore = 0.0;

  set_target_fps(60);

  while !window_should_close() {
    scrolling_back -= 0.1;
    scrolling_mid -= 0.5;
    scrolling_fore -= 1.0;

    if scrolling_back <= -background.width as f32 * 2.0 {
      scrolling_back = 0.0;
    }
    if scrolling_mid <= -midground.width as f32 * 2.0 {
      scrolling_mid = 0.0;
    }
    if scrolling_fore <= -foreground.width as f32 * 2.0 {
      scrolling_fore = 0.0;
    }

    begin_drawing();

    clear_background(get_color(0x052c46ff));

    draw_texture_ex(
      background,
      Vector2 {
        x: scrolling_back,
        y: 20.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );
    draw_texture_ex(
      background,
      Vector2 {
        x: background.width as f32 * 2.0 + scrolling_back,
        y: 20.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );

    draw_texture_ex(
      midground,
      Vector2 {
        x: scrolling_mid,
        y: 20.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );
    draw_texture_ex(
      midground,
      Vector2 {
        x: midground.width as f32 * 2 + scrolling_mid,
        y: 20.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );

    draw_texture_ex(
      foreground,
      Vector2 {
        x: scrolling_fore,
        y: 70.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );
    draw_texture_ex(
      foreground,
      Vector2 {
        x: foreground.width as f32 * 2.0 + scrolling_fore,
        y: 70.0,
      },
      0.0,
      2.0,
      colors::WHITE,
    );

    draw_text("BACKGROUND SCROLLING & PARALLAX", 10, 10, 20, colors::RED);
    draw_text(
      "(c) Cyberpunk Street Environment by Luis Zuno (@ansimuz)",
      SCREEN_WIDTH - 330,
      SCREEN_HEIGHT - 20,
      10,
      colors::RAYWHITE,
    );

    end_drawing();
  }

  unload_texture(background);
  unload_texture(midground);
  unload_texture(foreground);

  close_window();
}
