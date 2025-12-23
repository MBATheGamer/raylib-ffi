use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, get_fps, get_screen_height,
    get_screen_width, init_window,
    keyboard::is_key_pressed,
    mouse::{get_mouse_position, is_mouse_button_pressed},
    set_config_flags, set_target_fps, window_should_close,
  },
  enums::{ConfigFlags, KeyboardKey, MouseButton, TextureFilter},
  shape::{check_collision_point_rec, draw_rectangle, draw_rectangle_lines_ex, draw_rectangle_rec},
  structs::{Color, Rectangle, Texture, Vector2},
  text::draw_text,
  texture::{
    color_alpha, draw_texture, draw_texture_pro, load_texture, set_texture_filter, unload_texture,
  },
};

const OPT_WIDTH: i32 = 220;
const MARGIN_SIZE: i32 = 8;
const COLOR_SIZE: i32 = 16;

fn main() {
  const SCREEN_WIDTH: i32 = 800;
  const SCREEN_HEIGHT: i32 = 450;

  set_config_flags(&[ConfigFlags::WindowResizable]);
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - tiled drawing",
  );

  let tex_pattern = load_texture("resources/patterns.png");
  set_texture_filter(tex_pattern, TextureFilter::Trilinear);

  let rec_pattern: Vec<Rectangle> = vec![
    Rectangle {
      x: 3.0,
      y: 3.0,
      width: 66.0,
      height: 66.0,
    },
    Rectangle {
      x: 75.0,
      y: 3.0,
      width: 100.0,
      height: 100.0,
    },
    Rectangle {
      x: 3.0,
      y: 75.0,
      width: 66.0,
      height: 66.0,
    },
    Rectangle {
      x: 7.0,
      y: 156.0,
      width: 50.0,
      height: 50.0,
    },
    Rectangle {
      x: 85.0,
      y: 106.0,
      width: 90.0,
      height: 45.0,
    },
    Rectangle {
      x: 75.0,
      y: 154.0,
      width: 100.0,
      height: 60.0,
    },
  ];

  let colors: Vec<Color> = vec![
    colors::BLACK,
    colors::MAROON,
    colors::ORANGE,
    colors::BLUE,
    colors::PURPLE,
    colors::BEIGE,
    colors::LIME,
    colors::RED,
    colors::DARKGRAY,
    colors::SKYBLUE,
  ];
  let mut color_rec: Vec<Rectangle> = vec![];

  let mut x = 0;
  let mut y = 0;
  for i in 0..colors.len() {
    color_rec.push(Rectangle {
      x: (2 + MARGIN_SIZE + x) as f32,
      y: (22 + 256 + MARGIN_SIZE + y) as f32,
      width: (COLOR_SIZE * 2) as f32,
      height: COLOR_SIZE as f32,
    });
    if i == (colors.len() / 2 - 1) {
      x = 0;
      y += COLOR_SIZE + MARGIN_SIZE;
    } else {
      x += COLOR_SIZE * 2 + MARGIN_SIZE;
    }
  }

  let mut active_pattern = 0;
  let mut active_col = 0;
  let mut scale = 1.0;
  let mut rotation = 0.0;

  set_target_fps(60);

  while !window_should_close() {
    if is_mouse_button_pressed(MouseButton::Left) {
      let mouse = get_mouse_position();

      for i in 0..rec_pattern.len() {
        if check_collision_point_rec(
          mouse,
          Rectangle {
            x: 2.0 + MARGIN_SIZE as f32 + rec_pattern[i].x,
            y: 40.0 + MARGIN_SIZE as f32 + rec_pattern[i].y,
            width: rec_pattern[i].width,
            height: rec_pattern[i].height,
          },
        ) {
          active_pattern = i;
          break;
        }
      }

      for i in 0..colors.len() {
        if check_collision_point_rec(mouse, color_rec[i]) {
          active_col = i;
          break;
        }
      }
    }

    if is_key_pressed(KeyboardKey::KeyUp) {
      scale += 0.25;
    }
    if is_key_pressed(KeyboardKey::KeyDown) {
      scale -= 0.25;
    }
    if scale > 10.0 {
      scale = 10.0;
    } else if scale <= 0.0 {
      scale = 0.25;
    }

    if is_key_pressed(KeyboardKey::KeyLeft) {
      rotation -= 25.0;
    }
    if is_key_pressed(KeyboardKey::KeyRight) {
      rotation += 25.0;
    }

    if is_key_pressed(KeyboardKey::KeySpace) {
      rotation = 0.0;
      scale = 1.0;
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_texture_tiled(
      tex_pattern,
      rec_pattern[active_pattern],
      Rectangle {
        x: (OPT_WIDTH + MARGIN_SIZE) as f32,
        y: MARGIN_SIZE as f32,
        width: (get_screen_width() - OPT_WIDTH - 2 * MARGIN_SIZE) as f32,
        height: (get_screen_height() - 2 * MARGIN_SIZE) as f32,
      },
      Vector2 { x: 0.0, y: 0.0 },
      rotation,
      scale,
      colors[active_col],
    );

    draw_rectangle(
      MARGIN_SIZE,
      MARGIN_SIZE,
      OPT_WIDTH - MARGIN_SIZE,
      get_screen_height() - 2 * MARGIN_SIZE,
      color_alpha(colors::LIGHTGRAY, 0.5),
    );

    draw_text(
      "Select Pattern",
      2 + MARGIN_SIZE,
      30 + MARGIN_SIZE,
      10,
      colors::BLACK,
    );
    draw_texture(
      tex_pattern,
      2 + MARGIN_SIZE,
      40 + MARGIN_SIZE,
      colors::BLACK,
    );
    draw_rectangle(
      2 + MARGIN_SIZE + rec_pattern[active_pattern].x as i32,
      40 + MARGIN_SIZE + rec_pattern[active_pattern].y as i32,
      rec_pattern[active_pattern].width as i32,
      rec_pattern[active_pattern].height as i32,
      color_alpha(colors::DARKBLUE, 0.3),
    );

    draw_text(
      "Select Color",
      2 + MARGIN_SIZE,
      10 + 256 + MARGIN_SIZE,
      10,
      colors::BLACK,
    );
    for i in 0..colors.len() {
      draw_rectangle_rec(color_rec[i], colors[i]);
      if active_col == i {
        draw_rectangle_lines_ex(color_rec[i], 3.0, color_alpha(colors::WHITE, 0.5))
      };
    }

    draw_text(
      "Scale (UP/DOWN to change)",
      2 + MARGIN_SIZE,
      80 + 256 + MARGIN_SIZE,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!("{:.2}x", scale),
      2 + MARGIN_SIZE,
      92 + 256 + MARGIN_SIZE,
      20,
      colors::BLACK,
    );

    draw_text(
      "Rotation (LEFT/RIGHT to change)",
      2 + MARGIN_SIZE,
      122 + 256 + MARGIN_SIZE,
      10,
      colors::BLACK,
    );
    draw_text(
      &format!("{:.0} degrees", rotation),
      2 + MARGIN_SIZE,
      134 + 256 + MARGIN_SIZE,
      20,
      colors::BLACK,
    );

    draw_text(
      "Press [SPACE] to reset",
      2 + MARGIN_SIZE,
      164 + 256 + MARGIN_SIZE,
      10,
      colors::DARKBLUE,
    );

    draw_text(
      &format!("{} FPS", get_fps()),
      2 + MARGIN_SIZE,
      2 + MARGIN_SIZE,
      20,
      colors::BLACK,
    );
    end_drawing();
  }

  unload_texture(tex_pattern);

  close_window();
}

fn draw_texture_tiled(
  texture: Texture,
  source: Rectangle,
  dest: Rectangle,
  origin: Vector2,
  rotation: f32,
  scale: f32,
  tint: Color,
) {
  if texture.id <= 0 || scale <= 0.0 {
    return;
  }
  if source.width == 0.0 || source.height == 0.0 {
    return;
  }

  let tile_width = (source.width * scale) as i32;
  let tile_height = (source.height * scale) as i32;
  if dest.width < tile_width as f32 && dest.height < tile_height as f32 {
    draw_texture_pro(
      texture,
      Rectangle {
        x: source.x,
        y: source.y,
        width: (dest.width / tile_width as f32) * source.width,
        height: (dest.height / tile_height as f32) * source.height,
      },
      Rectangle {
        x: dest.x,
        y: dest.y,
        width: dest.width,
        height: dest.height,
      },
      origin,
      rotation,
      tint,
    );
  } else if dest.width <= tile_width as f32 {
    let mut dy = 0;
    while dy + tile_height < dest.height as i32 {
      draw_texture_pro(
        texture,
        Rectangle {
          x: source.x,
          y: source.y,
          width: (dest.width / tile_width as f32) * source.width,
          height: source.height,
        },
        Rectangle {
          x: dest.x,
          y: dest.y + dy as f32,
          width: dest.width,
          height: tile_height as f32,
        },
        origin,
        rotation,
        tint,
      );
      dy += tile_height;
    }

    if dy < dest.height as i32 {
      draw_texture_pro(
        texture,
        Rectangle {
          x: source.x,
          y: source.y,
          width: (dest.width / tile_width as f32) * source.width,
          height: ((dest.height - dy as f32) / tile_height as f32) * source.height,
        },
        Rectangle {
          x: dest.x,
          y: dest.y + dy as f32,
          width: dest.width,
          height: dest.height - dy as f32,
        },
        origin,
        rotation,
        tint,
      );
    }
  } else if dest.height <= tile_height as f32 {
    let mut dx = 0;
    while dx + tile_width < dest.width as i32 {
      draw_texture_pro(
        texture,
        Rectangle {
          x: source.x,
          y: source.y,
          width: source.width,
          height: (dest.height / tile_height as f32) * source.height,
        },
        Rectangle {
          x: dest.x + dx as f32,
          y: dest.y,
          width: tile_width as f32,
          height: dest.height,
        },
        origin,
        rotation,
        tint,
      );
      dx += tile_width;
    }

    if dx < dest.width as i32 {
      draw_texture_pro(
        texture,
        Rectangle {
          x: source.x,
          y: source.y,
          width: ((dest.width - dx as f32) / tile_width as f32) * source.width,
          height: (dest.height / tile_height as f32) * source.height,
        },
        Rectangle {
          x: dest.x + dx as f32,
          y: dest.y,
          width: dest.width - dx as f32,
          height: dest.height,
        },
        origin,
        rotation,
        tint,
      );
    }
  } else {
    let mut dx = 0;
    while dx + tile_width < dest.width as i32 {
      let mut dy = 0;
      while dy + tile_height < dest.height as i32 {
        draw_texture_pro(
          texture,
          source,
          Rectangle {
            x: dest.x + dx as f32,
            y: dest.y + dy as f32,
            width: tile_width as f32,
            height: tile_height as f32,
          },
          origin,
          rotation,
          tint,
        );
        dy += tile_height;
      }

      if dy < dest.height as i32 {
        draw_texture_pro(
          texture,
          Rectangle {
            x: source.x,
            y: source.y,
            width: source.width,
            height: ((dest.height - dy as f32) / tile_height as f32) * source.height,
          },
          Rectangle {
            x: dest.x + dx as f32,
            y: dest.y + dy as f32,
            width: tile_width as f32,
            height: dest.height - dy as f32,
          },
          origin,
          rotation,
          tint,
        );
      }

      dx += tile_width;
    }

    if dx < dest.width as i32 {
      let mut dy = 0;
      while dy + tile_height < dest.height as i32 {
        draw_texture_pro(
          texture,
          Rectangle {
            x: source.x,
            y: source.y,
            width: ((dest.width - dx as f32) / tile_width as f32) * source.width,
            height: source.height,
          },
          Rectangle {
            x: dest.x + dx as f32,
            y: dest.y + dy as f32,
            width: dest.width - dx as f32,
            height: tile_height as f32,
          },
          origin,
          rotation,
          tint,
        );
        dy += tile_height;
      }

      if dy < dest.height as i32 {
        draw_texture_pro(
          texture,
          Rectangle {
            x: source.x,
            y: source.y,
            width: ((dest.width - dx as f32) / tile_width as f32) * source.width,
            height: ((dest.height - dy as f32) / tile_height as f32) * source.height,
          },
          Rectangle {
            x: dest.x + dx as f32,
            y: dest.y + dy as f32,
            width: dest.width - dx as f32,
            height: dest.height - dy as f32,
          },
          origin,
          rotation,
          tint,
        );
      }
    }
  }
}
