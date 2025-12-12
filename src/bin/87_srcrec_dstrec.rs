use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
    window_should_close,
  },
  shape::draw_line,
  structs::{Rectangle, Vector2},
  text::draw_text,
  texture::{draw_texture_pro, load_texture, unload_texture},
};

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - srcrec dstrec",
  );

  let scarfy = load_texture("resources/scarfy.png");

  let frame_width = scarfy.width / 6;
  let frame_height = scarfy.height;

  let source_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: frame_width as f32,
    height: frame_height as f32,
  };

  let dest_rec = Rectangle {
    x: SCREEN_WIDTH as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0,
    width: frame_width as f32 * 2.0,
    height: frame_height as f32 * 2.0,
  };

  let origin = Vector2 {
    x: frame_width as f32,
    y: frame_height as f32,
  };

  let mut rotation = 0;

  set_target_fps(60);

  while !window_should_close() {
    rotation += 1;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture_pro(
      scarfy,
      source_rec,
      dest_rec,
      origin,
      rotation as f32,
      colors::WHITE,
    );

    draw_line(
      dest_rec.x as i32,
      0,
      dest_rec.x as i32,
      SCREEN_HEIGHT,
      colors::GRAY,
    );
    draw_line(
      0,
      dest_rec.y as i32,
      SCREEN_WIDTH,
      dest_rec.y as i32,
      colors::GRAY,
    );

    draw_text(
      "(c) Scarfy sprite by Eiden Marsal",
      SCREEN_WIDTH - 200,
      SCREEN_HEIGHT - 20,
      10,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_texture(scarfy);

  close_window();
}
