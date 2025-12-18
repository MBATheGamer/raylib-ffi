use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, begin_texture_mode, clear_background, close_window, end_drawing,
    end_texture_mode, get_screen_height, get_screen_width, init_window, is_window_resized,
    mouse::{get_mouse_position, is_mouse_button_pressed},
    set_config_flags, set_target_fps, window_should_close,
  },
  enums::{ConfigFlags, MouseButton},
  shape::{check_collision_point_rec, draw_circle, draw_rectangle_lines, draw_rectangle_rec},
  structs::{Rectangle, RenderTexture, Vector2},
  text::draw_text,
  texture::{draw_texture_pro, fade, load_render_texture, unload_render_texture},
};

const RESOLUTION_COUNT: usize = 4;
const VIEWPORT_TYPE_COUNT: usize = 6;

#[derive(Clone, Copy)]
enum ViewportType {
  KeepAspectInteger = 0,
  KeepHeightInteger,
  KeepWidthInteger,
  KeepAspect,
  KeepHeight,
  KeepWidth,
}

fn get(value: i32) -> ViewportType {
  match value {
    1 => ViewportType::KeepHeightInteger,
    2 => ViewportType::KeepWidthInteger,
    3 => ViewportType::KeepAspect,
    4 => ViewportType::KeepHeight,
    5 => ViewportType::KeepWidth,
    _ => ViewportType::KeepAspectInteger,
  }
}

fn main() {
  let resolution_list: Vec<Vector2> = vec![
    Vector2 { x: 64.0, y: 64.0 },
    Vector2 { x: 256.0, y: 240.0 },
    Vector2 { x: 320.0, y: 180.0 },
    Vector2 {
      x: 3840.0,
      y: 2160.0,
    },
  ];
  let mut resolution_index = 0;

  let mut screen_width = 800;
  let mut screen_height = 450;
  let mut game_width = 64;
  let mut game_height = 64;

  let mut target = RenderTexture::default();
  let mut source_rect = Rectangle::default();
  let mut dest_rect = Rectangle::default();

  let viewport_type_names: Vec<&str> = vec![
    "KEEP_ASPECT_INTEGER",
    "KEEP_HEIGHT_INTEGER",
    "KEEP_WIDTH_INTEGER",
    "KEEP_ASPECT",
    "KEEP_HEIGHT",
    "KEEP_WIDTH",
  ];
  let mut viewport_type = ViewportType::KeepAspectInteger;

  set_config_flags(&[ConfigFlags::WindowResizable]);
  init_window(
    screen_width,
    screen_height,
    "raylib [core] example - viewport scaling",
  );
  resize_render_size(
    viewport_type,
    &mut screen_width,
    &mut screen_height,
    game_width,
    game_height,
    &mut source_rect,
    &mut dest_rect,
    &mut target,
  );

  set_target_fps(60);

  let decrease_resolution_button = Rectangle {
    x: 200.0,
    y: 30.0,
    width: 10.0,
    height: 10.0,
  };
  let increase_resolution_button = Rectangle {
    x: 215.0,
    y: 30.0,
    width: 10.0,
    height: 10.0,
  };
  let decrease_type_button = Rectangle {
    x: 200.0,
    y: 45.0,
    width: 10.0,
    height: 10.0,
  };
  let increase_type_button = Rectangle {
    x: 215.0,
    y: 45.0,
    width: 10.0,
    height: 10.0,
  };

  while !window_should_close() {
    if is_window_resized() {
      resize_render_size(
        viewport_type,
        &mut screen_width,
        &mut screen_height,
        game_width,
        game_height,
        &mut source_rect,
        &mut dest_rect,
        &mut target,
      );
    }
    let mouse_position = get_mouse_position();
    let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);

    if check_collision_point_rec(mouse_position, decrease_resolution_button) && mouse_pressed {
      resolution_index = (resolution_index + RESOLUTION_COUNT - 1) % RESOLUTION_COUNT;
      game_width = resolution_list[resolution_index].x as i32;
      game_height = resolution_list[resolution_index].y as i32;
      resize_render_size(
        viewport_type,
        &mut screen_width,
        &mut screen_height,
        game_width,
        game_height,
        &mut source_rect,
        &mut dest_rect,
        &mut target,
      );
    }
    if check_collision_point_rec(mouse_position, increase_resolution_button) && mouse_pressed {
      resolution_index = (resolution_index + 1) % RESOLUTION_COUNT;
      game_width = resolution_list[resolution_index].x as i32;
      game_height = resolution_list[resolution_index].y as i32;
      resize_render_size(
        viewport_type,
        &mut screen_width,
        &mut screen_height,
        game_width,
        game_height,
        &mut source_rect,
        &mut dest_rect,
        &mut target,
      );
    }
    if check_collision_point_rec(mouse_position, decrease_type_button) && mouse_pressed {
      viewport_type =
        get((viewport_type as i32 + VIEWPORT_TYPE_COUNT as i32 - 1) % VIEWPORT_TYPE_COUNT as i32);
      resize_render_size(
        viewport_type,
        &mut screen_width,
        &mut screen_height,
        game_width,
        game_height,
        &mut source_rect,
        &mut dest_rect,
        &mut target,
      );
    }
    if check_collision_point_rec(mouse_position, increase_type_button) && mouse_pressed {
      viewport_type = get((viewport_type as i32 + 1) % VIEWPORT_TYPE_COUNT as i32);
      resize_render_size(
        viewport_type,
        &mut screen_width,
        &mut screen_height,
        game_width,
        game_height,
        &mut source_rect,
        &mut dest_rect,
        &mut target,
      );
    }

    let texture_mouse_position =
      screen_2_render_texture_position(mouse_position, &source_rect, &dest_rect);

    begin_texture_mode(target);
    clear_background(colors::WHITE);
    draw_circle(
      texture_mouse_position.x as i32,
      texture_mouse_position.y as i32,
      20.0,
      colors::LIME,
    );

    end_texture_mode();

    begin_drawing();
    clear_background(colors::BLACK);

    const ORIGIN_POSITION: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const ROTATION: f32 = 0.0;
    draw_texture_pro(
      target.texture,
      source_rect,
      dest_rect,
      ORIGIN_POSITION,
      ROTATION,
      colors::WHITE,
    );

    let info_rect = Rectangle {
      x: 5.0,
      y: 5.0,
      width: 330.0,
      height: 105.0,
    };
    draw_rectangle_rec(info_rect, fade(colors::LIGHTGRAY, 0.7));
    draw_rectangle_lines(
      info_rect.x as i32,
      info_rect.y as i32,
      info_rect.width as i32,
      info_rect.height as i32,
      colors::BLUE,
    );

    draw_text(
      &format!("Window Resolution: {} x {}", screen_width, screen_height),
      15,
      15,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!("Game Resolution: {} x {}", game_width, game_height),
      15,
      30,
      10,
      colors::BLACK,
    );

    draw_text(
      &format!("Type: {}", viewport_type_names[viewport_type as usize]),
      15,
      45,
      10,
      colors::BLACK,
    );
    let scale_ratio = Vector2 {
      x: dest_rect.width / source_rect.width,
      y: dest_rect.height / -source_rect.height,
    };
    if scale_ratio.x < 0.001 || scale_ratio.y < 0.001 {
      draw_text(&format!("Scale ratio: INVALID"), 15, 60, 10, colors::BLACK);
    } else {
      draw_text(
        &format!("Scale ratio: {:.2} x {:.2}", scale_ratio.x, scale_ratio.y),
        15,
        60,
        10,
        colors::BLACK,
      );
    }
    draw_text(
      &format!(
        "Source size: {:.2} x {:.2}",
        source_rect.width, -source_rect.height
      ),
      15,
      75,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!(
        "Destination size: {:.2} x {:.2}",
        dest_rect.width, dest_rect.height
      ),
      15,
      90,
      10,
      colors::BLACK,
    );

    draw_rectangle_rec(decrease_type_button, colors::SKYBLUE);
    draw_rectangle_rec(increase_type_button, colors::SKYBLUE);
    draw_rectangle_rec(decrease_resolution_button, colors::SKYBLUE);
    draw_rectangle_rec(increase_resolution_button, colors::SKYBLUE);
    draw_text(
      "<",
      decrease_type_button.x as i32 + 3,
      decrease_type_button.y as i32 + 1,
      10,
      colors::BLACK,
    );
    draw_text(
      ">",
      increase_type_button.x as i32 + 3,
      increase_type_button.y as i32 + 1,
      10,
      colors::BLACK,
    );
    draw_text(
      "<",
      decrease_resolution_button.x as i32 + 3,
      decrease_resolution_button.y as i32 + 1,
      10,
      colors::BLACK,
    );
    draw_text(
      ">",
      increase_resolution_button.x as i32 + 3,
      increase_resolution_button.y as i32 + 1,
      10,
      colors::BLACK,
    );

    end_drawing();
  }

  close_window();
}

fn keep_aspect_centered_integer(
  screen_width: i32,
  screen_height: i32,
  game_width: i32,
  game_height: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  source_rect.x = 0.;
  source_rect.y = game_height as f32;
  source_rect.width = game_width as f32;
  source_rect.height = -game_height as f32;

  let ratio_x = screen_width / game_width;
  let ratio_y = screen_height / game_height;
  let resize_ratio = (if ratio_x < ratio_y { ratio_x } else { ratio_y }) as f32;

  dest_rect.x = ((screen_width as f32 - game_width as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.y = ((screen_height as f32 - game_height as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.width = (game_width as f32 * resize_ratio) as i32 as f32;
  dest_rect.height = (game_height as f32 * resize_ratio) as i32 as f32;
}

fn keep_height_centered_integer(
  screen_width: i32,
  screen_height: i32,
  _: i32,
  game_height: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  let resize_ratio = (screen_height / game_height) as f32;
  source_rect.x = 0.0;
  source_rect.y = 0.0;
  source_rect.width = (screen_width as f32 / resize_ratio) as i32 as f32;
  source_rect.height = -game_height as f32;

  dest_rect.x = ((screen_width as f32 - source_rect.width * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.y = ((screen_height as f32 - game_height as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.width = (source_rect.width * resize_ratio) as i32 as f32;
  dest_rect.height = (game_height as f32 * resize_ratio) as i32 as f32;
}

fn keep_width_centered_integer(
  screen_width: i32,
  screen_height: i32,
  game_width: i32,
  _: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  let resize_ratio = (screen_width / game_width) as f32;
  source_rect.x = 0.0;
  source_rect.y = 0.0;
  source_rect.width = game_width as f32;
  source_rect.height = (screen_height as f32 / resize_ratio) as i32 as f32;

  dest_rect.x = ((screen_width as f32 - game_width as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.y = ((screen_height as f32 - source_rect.height * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.width = (game_width as f32 * resize_ratio) as i32 as f32;
  dest_rect.height = (source_rect.height * resize_ratio) as i32 as f32;

  source_rect.height *= -1.0;
}

fn keep_aspect_centered(
  screen_width: i32,
  screen_height: i32,
  game_width: i32,
  game_height: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  source_rect.x = 0.0;
  source_rect.y = game_height as f32;
  source_rect.width = game_width as f32;
  source_rect.height = -game_height as f32;

  let ratio_x = screen_width as f32 / game_width as f32;
  let ratio_y = screen_height as f32 / game_height as f32;
  let resize_ratio = (if ratio_x < ratio_y { ratio_x } else { ratio_y }) as f32;

  dest_rect.x = ((screen_width as f32 - game_width as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.y = ((screen_height as f32 - game_height as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.width = (game_width as f32 * resize_ratio) as i32 as f32;
  dest_rect.height = (game_height as f32 * resize_ratio) as i32 as f32;
}

fn keep_height_centered(
  screen_width: i32,
  screen_height: i32,
  _: i32,
  game_height: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  let resize_ratio = screen_height as f32 / game_height as f32;
  source_rect.x = 0.0;
  source_rect.y = 0.0;
  source_rect.width = (screen_width as f32 / resize_ratio as f32) as i32 as f32;
  source_rect.height = -game_height as f32;

  dest_rect.x = ((screen_width as f32 - source_rect.width * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.y = ((screen_height as f32 - game_height as f32 * resize_ratio) * 0.5) as i32 as f32;
  dest_rect.width = (source_rect.width * resize_ratio) as i32 as f32;
  dest_rect.height = (game_height as f32 * resize_ratio) as i32 as f32;
}

fn keep_width_centered(
  screen_width: i32,
  screen_height: i32,
  game_width: i32,
  _: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
) {
  let resize_ratio = screen_width as f32 / game_width as f32;
  source_rect.x = 0.0;
  source_rect.y = 0.0;
  source_rect.width = game_width as f32;
  source_rect.height = (screen_height as f32 / resize_ratio as f32) as i32 as f32;

  dest_rect.x = ((screen_width as f32 - game_width as f32 * resize_ratio) * 0.5) as f32;
  dest_rect.y = ((screen_height as f32 - source_rect.height * resize_ratio) * 0.5) as f32;
  dest_rect.width = (game_width as f32 * resize_ratio) as f32;
  dest_rect.height = (source_rect.height * resize_ratio) as f32;

  source_rect.height *= -1.0;
}

fn resize_render_size(
  viewport_type: ViewportType,
  screen_width: &mut i32,
  screen_height: &mut i32,
  game_width: i32,
  game_height: i32,
  source_rect: &mut Rectangle,
  dest_rect: &mut Rectangle,
  target: &mut RenderTexture,
) {
  *screen_width = get_screen_width();
  *screen_height = get_screen_height();

  match viewport_type {
    ViewportType::KeepAspectInteger => {
      keep_aspect_centered_integer(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
    ViewportType::KeepHeightInteger => {
      keep_height_centered_integer(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
    ViewportType::KeepWidthInteger => {
      keep_width_centered_integer(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
    ViewportType::KeepAspect => {
      keep_aspect_centered(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
    ViewportType::KeepHeight => {
      keep_height_centered(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
    ViewportType::KeepWidth => {
      keep_width_centered(
        *screen_width,
        *screen_height,
        game_width,
        game_height,
        source_rect,
        dest_rect,
      );
    }
  }
  unload_render_texture(*target);
  *target = load_render_texture(source_rect.width as i32, -source_rect.height as i32);
}

fn screen_2_render_texture_position(
  point: Vector2,
  texture_rect: &Rectangle,
  scaled_rect: &Rectangle,
) -> Vector2 {
  let relative_position = Vector2 {
    x: point.x - scaled_rect.x,
    y: point.y - scaled_rect.y,
  };
  let ratio = Vector2 {
    x: texture_rect.width / scaled_rect.width,
    y: -texture_rect.height / scaled_rect.height,
  };

  return Vector2 {
    x: relative_position.x * ratio.x,
    y: relative_position.y * ratio.x,
  };
}
