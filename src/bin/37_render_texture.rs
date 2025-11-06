fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - render texture",
  );

  let render_texture_width = 300;
  let render_texture_height = 300;
  let target = load_render_texture(render_texture_width, render_texture_height);

  let ball_position = Vector2 {
    x: render_texture_width as f32 / 2.0,
    y: render_texture_height as f32 / 2.0,
  };
  let ball_speed = Vector2 { x: 5.0, y: 4.0 };
  let ball_radius = 20;

  let rotation = 0.0;

  set_target_fps(60);

  while !window_should_close() {
    ball_position.x += ball_speed.x;
    ball_position.y += ball_speed.y;

    if ball_position.x as i32 >= (render_texture_width - ball_radius)
      || ball_position.x as i32 <= ball_radius
    {
      ball_speed.x *= -1.0;
    }
    if ball_position.y as i32 >= (render_texture_height - ball_radius)
      || ball_position.y as i32 <= ball_radius
    {
      ball_speed.y *= -1.0;
    }

    rotation += 0.5;

    begin_texture_mode(target);

    clear_background(colors::SKYBLUE);

    draw_rectangle(0, 0, 20, 20, colors::RED);
    draw_circle_v(ball_position, ball_radius as f32, colors::MAROON);

    end_texture_mode();

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture_pro(
      target.texture,
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: target.texture.width as f32,
        height: -target.texture.height as f32,
      },
      Rectangle {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0,
        width: target.texture.width as f32,
        height: target.texture.height as f32,
      },
      Vector2 {
        x: target.texture.width as f32 / 2.0,
        y: target.texture.height as f32 / 2.0,
      },
      rotation,
      colors::WHITE,
    );

    draw_text(
      "DRAWING BOUNCING BALL INSIDE RENDER TEXTURE!",
      10,
      SCREEN_HEIGHT - 40,
      20,
      colors::BLACK,
    );

    draw_fps(10, 10);

    end_drawing();
  }

  close_window();
}
