use std::env;

use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_mode_3d, begin_shader_mode, begin_texture_mode, begin_vr_stereo_mode,
    clear_background, close_window, disable_cursor, end_drawing, end_mode_3d, end_shader_mode,
    end_texture_mode, end_vr_stereo_mode, get_screen_height, get_screen_width, get_shader_location,
    init_window, load_shader, load_vr_stereo_config, set_shader_value, set_target_fps,
    unload_shader, unload_vr_stereo_config, update_camera, window_should_close,
  },
  enums::{CameraMode, CameraProjection, ShaderUniformType},
  model::{draw_cube, draw_cube_wires, draw_grid},
  structs::{Camera3D, Rectangle, Vector2, Vector3, VrDeviceInfo},
  text::draw_fps,
  texture::{draw_texture_pro, load_render_texture, unload_render_texture},
};

fn main() {
  let platform = match env::var("PLATFORM") {
    Ok(platform) => platform,
    _ => format!("DESKTOP"),
  };
  let glsl_version = if platform == "DESKTOP" { 330 } else { 100 };
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - vr simulator",
  );

  let device = VrDeviceInfo {
    h_resolution: 2160,
    v_resolution: 1200,
    h_screen_size: 0.133793,
    v_screen_size: 0.0669,
    eye_to_screen_distance: 0.041,
    lens_separation_distance: 0.07,
    interpupillary_distance: 0.07,

    lens_distortion_values: [1.0, 0.22, 0.24, 0.0],
    chroma_ab_correction: [0.996, -0.004, 1.014, 0.0],
  };

  let config = load_vr_stereo_config(device);

  let distortion = load_shader(
    "",
    &format!("resources/shaders/glsl{}/distortion.fs", glsl_version),
  );

  set_shader_value(
    distortion,
    get_shader_location(distortion, "leftLensCenter"),
    &config.left_lens_center,
    ShaderUniformType::Vec2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "rightLensCenter"),
    &config.right_lens_center,
    ShaderUniformType::Vec2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "leftScreenCenter"),
    &config.left_screen_center,
    ShaderUniformType::Vec2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "rightScreenCenter"),
    &config.right_screen_center,
    ShaderUniformType::Vec2,
  );

  set_shader_value(
    distortion,
    get_shader_location(distortion, "scale"),
    &config.scale,
    ShaderUniformType::Vec2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "scaleIn"),
    &config.scale_in,
    ShaderUniformType::Vec2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "deviceWarpParam"),
    &device.lens_distortion_values,
    ShaderUniformType::Vec4,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "chromaAbParam"),
    &device.chroma_ab_correction,
    ShaderUniformType::Vec4,
  );

  let target = load_render_texture(device.h_resolution, device.v_resolution);

  let source_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: target.texture.width as f32,
    height: -target.texture.height as f32,
  };
  let dest_rec = Rectangle {
    x: 0.0,
    y: 0.0,
    width: get_screen_width() as f32,
    height: get_screen_height() as f32,
  };

  let mut camera = Camera3D {
    position: Vector3 {
      x: 5.0,
      y: 2.0,
      z: 5.0,
    },
    target: Vector3 {
      x: 0.0,
      y: 2.0,
      z: 0.0,
    },
    up: Vector3 {
      x: 0.0,
      y: 1.0,
      z: 0.0,
    },
    fovy: 60.0,
    projection: CameraProjection::Perspective,
  };
  let cube_position = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };

  disable_cursor();

  set_target_fps(60);

  while !window_should_close() {
    update_camera(&mut camera, CameraMode::FirstPerson);

    begin_texture_mode(target);
    clear_background(colors::RAYWHITE);
    begin_vr_stereo_mode(config);
    begin_mode_3d(camera);

    draw_cube(cube_position, 2.0, 2.0, 2.0, colors::RED);
    draw_cube_wires(cube_position, 2.0, 2.0, 2.0, colors::MAROON);
    draw_grid(40, 1.0);

    end_mode_3d();
    end_vr_stereo_mode();
    end_texture_mode();

    begin_drawing();
    clear_background(colors::RAYWHITE);
    begin_shader_mode(distortion);
    draw_texture_pro(
      target.texture,
      source_rec,
      dest_rec,
      Vector2 { x: 0.0, y: 0.0 },
      0.0,
      colors::WHITE,
    );
    end_shader_mode();
    draw_fps(10, 10);
    end_drawing();
  }

  unload_vr_stereo_config(config);

  unload_render_texture(target);
  unload_shader(distortion);

  close_window();
}
