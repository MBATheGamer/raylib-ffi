use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode, get_frame_time, init_window, set_config_flags, set_target_fps,
    window_should_close,
  },
  enums::{ConfigFlags, TextureFilter},
  shape::{draw_circle_v, draw_line_ex, draw_rectangle, draw_rectangle_pro},
  structs::{Rectangle, Vector2},
  texture::{
    draw_texture_rec, fade, load_render_texture, set_texture_filter, unload_render_texture,
  },
};

const SIMULATION_STEPS: i32 = 30;
const G: f32 = 9.81;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::WindowHighDPI]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [shapes] example - double pendulum",
  );

  let l1 = 15.0;
  let m1 = 0.2;
  let mut theta1 = 170.0f32.to_radians();
  let mut w1 = 0.0;
  let l2 = 15.0;
  let m2 = 0.1;
  let mut theta2 = 0.0f32.to_radians();
  let mut w2 = 0.0;
  let length_scaler = 0.1;
  let total_m = m1 + m2;

  let mut previous_position = calculate_double_pendulum_end_point(l1, theta1, l2, theta2);
  previous_position.x += (SCREEN_WIDTH / 2) as f32;
  previous_position.y += (SCREEN_HEIGHT / 2 - 100) as f32;

  let l_1 = l1 * length_scaler;
  let l_2 = l2 * length_scaler;

  let line_thick = 20.0;
  let trail_thick = 2.0;
  let fate_alpha = 0.01;

  let target = load_render_texture(SCREEN_WIDTH, SCREEN_HEIGHT);
  set_texture_filter(target.texture, TextureFilter::TextureFilterBilinear);

  set_target_fps(60);

  while !window_should_close() {
    let dt = get_frame_time();
    let step = dt / SIMULATION_STEPS as f32;
    let step2 = step * step;

    for _ in 0..SIMULATION_STEPS {
      let delta = theta1 - theta2;
      let sin_d = delta.sin();
      let cos_d = delta.cos();
      let cos_2d = (2.0 * delta).cos();
      let ww1 = w1 * w1;
      let ww2 = w2 * w2;

      let a1 = (-G * (2.0 * m1 + m2) * theta1.sin()
        - m2 * G * (theta1 - 2.0 * theta2).sin()
        - 2.0 * sin_d * m2 * (ww2 * l_2 + ww1 * l_1 * cos_d))
        / (l_1 * (2.0 * m1 + m2 - m2 * cos_2d));

      let a2 =
        (2.0 * sin_d * (ww1 * l_1 * total_m + G * total_m * theta1.cos() + ww2 * l_2 * m2 * cos_d))
          / (l_2 * (2.0 * m1 + m2 - m2 * cos_2d));

      theta1 += w1 * step + 0.5 * a1 * step2;
      theta2 += w2 * step + 0.5 * a2 * step2;

      w1 += a1 * step;
      w2 += a2 * step;
    }

    let mut current_position = calculate_double_pendulum_end_point(l1, theta1, l2, theta2);
    current_position.x += (SCREEN_WIDTH / 2) as f32;
    current_position.y += (SCREEN_HEIGHT / 2 - 100) as f32;

    begin_texture_mode(target);

    draw_rectangle(
      0,
      0,
      SCREEN_WIDTH,
      SCREEN_HEIGHT,
      fade(colors::BLACK, fate_alpha),
    );

    draw_circle_v(previous_position, trail_thick, colors::RED);
    draw_line_ex(
      previous_position,
      current_position,
      trail_thick * 2.0,
      colors::RED,
    );
    end_texture_mode();

    previous_position = current_position;

    begin_drawing();

    clear_background(colors::BLACK);

    draw_texture_rec(
      target.texture,
      Rectangle {
        x: 0.0,
        y: 0.0,
        width: target.texture.width as f32,
        height: -target.texture.height as f32,
      },
      Vector2 { x: 0.0, y: 0.0 },
      colors::WHITE,
    );

    draw_rectangle_pro(
      Rectangle {
        x: SCREEN_WIDTH as f32 / 2.0,
        y: SCREEN_HEIGHT as f32 / 2.0 - 100.0,
        width: 10.0 * l1,
        height: line_thick,
      },
      Vector2 {
        x: 0.0,
        y: line_thick * 0.5,
      },
      90.0 - theta1.to_degrees(),
      colors::RAYWHITE,
    );

    let endpoint1 = calculate_pendulum_end_point(l1, theta1);
    draw_rectangle_pro(
      Rectangle {
        x: SCREEN_WIDTH as f32 / 2.0 + endpoint1.x,
        y: SCREEN_HEIGHT as f32 / 2.0 - 100.0 + endpoint1.y,
        width: 10.0 * l2,
        height: line_thick,
      },
      Vector2 {
        x: 0.0,
        y: line_thick * 0.5,
      },
      90.0 - theta2.to_degrees(),
      colors::RAYWHITE,
    );

    end_drawing();
  }

  unload_render_texture(target);

  close_window();
}

fn calculate_pendulum_end_point(l: f32, theta: f32) -> Vector2 {
  return Vector2 {
    x: 10.0 * l * theta.sin(),
    y: 10.0 * l * theta.cos(),
  };
}

fn calculate_double_pendulum_end_point(l1: f32, theta1: f32, l2: f32, theta2: f32) -> Vector2 {
  let endpoint1 = calculate_pendulum_end_point(l1, theta1);
  let endpoint2 = calculate_pendulum_end_point(l2, theta2);
  return Vector2 {
    x: endpoint1.x + endpoint2.x,
    y: endpoint1.y + endpoint2.y,
  };
}
