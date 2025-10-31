fn main() {
  let platform = env::var("PLATFORM").unwrap();
  let glsl_version = if platform == "DESKTOP" { 330 } else { 100 };
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [core] example - vr simulator",
  );

  let device = VrDeviceInfo {
    h_resolution: 2160,             // Horizontal resolution in pixels
    v_resolution: 1200,             // Vertical resolution in pixels
    h_screen_size: 0.133793,        // Horizontal size in meters
    v_screen_size: 0.0669,          // Vertical size in meters
    eye_to_screen_distance: 0.041,  // Distance between eye and display in meters
    lens_separation_distance: 0.07, // Lens separation distance in meters
    interpupillary_distance: 0.07,  // IPD (distance between pupils) in meters

    lens_distortion_values: [1.0, 0.22, 0.24, 0.0],
    chroma_ab_correction: [0.996, -0.004, 1.014, 0.0],
  };

  let config = load_vr_stereo_config(device);

  let distortion = load_shader(
    0,
    &format!("resources/shaders/glsl{}/distortion.fs", glsl_version),
  );

  set_shader_value(
    distortion,
    get_shader_location(distortion, "leftLensCenter"),
    config.leftLensCenter,
    SHADER_UNIFORM_VEC2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "rightLensCenter"),
    config.rightLensCenter,
    SHADER_UNIFORM_VEC2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "leftScreenCenter"),
    config.leftScreenCenter,
    SHADER_UNIFORM_VEC2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "rightScreenCenter"),
    config.rightScreenCenter,
    SHADER_UNIFORM_VEC2,
  );

  set_shader_value(
    distortion,
    get_shader_location(distortion, "scale"),
    config.scale,
    SHADER_UNIFORM_VEC2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "scaleIn"),
    config.scaleIn,
    SHADER_UNIFORM_VEC2,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "deviceWarpParam"),
    device.lens_distortion_values,
    SHADER_UNIFORM_VEC4,
  );
  set_shader_value(
    distortion,
    get_shader_location(distortion, "chromaAbParam"),
    device.chroma_ab_correction,
    SHADER_UNIFORM_VEC4,
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

  let camera = Camera3D {
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
    //----------------------------------------------------------------------------------
  }

  unload_vr_stereo_config(config);

  unload_render_texture(target);
  unload_shader(distortion);

  close_window();
}
