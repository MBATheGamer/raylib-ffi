use raylib_ffi::{
  audio::{close_audio_device, init_audio_device, load_sound, play_sound, unload_sound},
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    mouse::{get_mouse_position, is_mouse_button_down, is_mouse_button_released},
    set_target_fps, window_should_close,
  },
  enums::MouseButton,
  shape::check_collision_point_rec,
  structs::{Rectangle, Vector2},
  texture::{draw_texture_rec, load_texture, unload_texture},
};

const NUM_FRAMES: i32 = 3;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - sprite button",
  );

  init_audio_device();

  let fx_button = load_sound("resources/buttonfx.wav");
  let button = load_texture("resources/button.png");

  let frame_height = button.height as f32 / NUM_FRAMES as f32;
  let mut source_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: button.width as f32,
    height: frame_height as f32,
  };

  let btn_bounds = Rectangle {
    x: SCREEN_WIDTH as f32 / 2.0 - button.width as f32 / 2.0,
    y: SCREEN_HEIGHT as f32 / 2.0 - button.height as f32 / NUM_FRAMES as f32 / 2.0,
    width: button.width as f32,
    height: frame_height,
  };

  let mut btn_state;

  set_target_fps(60);

  while !window_should_close() {
    let mouse_point = get_mouse_position();
    let mut btn_action = false;

    if check_collision_point_rec(mouse_point, btn_bounds) {
      if is_mouse_button_down(MouseButton::Left) {
        btn_state = 2;
      } else {
        btn_state = 1;
      }

      if is_mouse_button_released(MouseButton::Left) {
        btn_action = true;
      }
    } else {
      btn_state = 0;
    }

    if btn_action {
      play_sound(fx_button);
    }

    source_rec.y = btn_state as f32 * frame_height;

    begin_drawing();

    clear_background(colors::RAYWHITE);

    draw_texture_rec(
      button,
      source_rec,
      Vector2 {
        x: btn_bounds.x,
        y: btn_bounds.y,
      },
      colors::WHITE,
    );

    end_drawing();
  }

  unload_texture(button);
  unload_sound(fx_button);

  close_audio_device();

  close_window();
}
