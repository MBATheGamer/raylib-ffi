use raylib_ffi::{
  consts::colors,
  core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window,
    mouse::{get_mouse_position, is_mouse_button_pressed},
    set_target_fps, window_should_close,
  },
  enums::MouseButton,
  shape::{draw_rectangle, draw_rectangle_lines, draw_rectangle_lines_ex},
  structs::{Color, Image, Rectangle},
  text::draw_text,
  texture::{
    draw_texture, gen_image_color, get_image_color, image_clear_background, image_draw_pixel,
    load_texture_from_image, unload_image, unload_texture, update_texture,
  },
};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const IMAGE_WIDTH: i32 = 800;
const IMAGE_HEIGHT: i32 = 800 / 2;

const DRAW_RULE_START_X: i32 = 585;
const DRAW_RULE_START_Y: i32 = 10;
const DRAW_RULE_SPACING: i32 = 15;
const DRAW_RULE_GROUP_SPACING: i32 = 50;
const DRAW_RULE_SIZE: i32 = 14;
const DRAW_RULE_INNER_SIZE: i32 = 10;

const PRESETS_SIZE_X: i32 = 42;
const PRESETS_SIZE_Y: i32 = 22;

const LINES_UPDATED_PER_FRAME: i32 = 4;

fn compute_line(image: &mut Image, line: i32, rule: i32) {
  for i in 1..(IMAGE_WIDTH - 1) {
    let prev_value = (if get_image_color(*image, i - 1, line - 1).red < 5 {
      4
    } else {
      0
    }) + (if get_image_color(*image, i, line - 1).red < 5 {
      2
    } else {
      0
    }) + (if get_image_color(*image, i + 1, line - 1).red < 5 {
      1
    } else {
      0
    });

    let curr_value = (rule & (1 << prev_value)) != 0;

    image_draw_pixel(
      image,
      i,
      line,
      if curr_value {
        colors::BLACK
      } else {
        colors::RAYWHITE
      },
    );
  }
}

fn main() {
  init_window(
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    "raylib [textures] example - cellular automata",
  );

  let mut image = gen_image_color(IMAGE_WIDTH, IMAGE_HEIGHT, colors::RAYWHITE);

  image_draw_pixel(&mut image, IMAGE_WIDTH / 2, 0, colors::BLACK);

  let texture = load_texture_from_image(image);

  let preset_values: Vec<i32> = vec![18, 30, 60, 86, 102, 124, 126, 150, 182, 225];

  let mut rule = 30;
  let mut line = 1;

  set_target_fps(60);

  while !window_should_close() {
    let mouse = get_mouse_position();
    let mut mouse_in_cell = -1;

    for i in 0..8 {
      let cell_x = DRAW_RULE_START_X - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING;
      let cell_y = DRAW_RULE_START_Y + DRAW_RULE_SPACING;
      if mouse.x >= cell_x as f32
        && mouse.x <= (cell_x + DRAW_RULE_SIZE) as f32
        && mouse.y >= cell_y as f32
        && mouse.y <= (cell_y + DRAW_RULE_SIZE) as f32
      {
        mouse_in_cell = i;
        break;
      }
    }

    if mouse_in_cell < 0 {
      for i in 0..preset_values.len() as i32 {
        let cell_x = 4 + (PRESETS_SIZE_X + 2) * (i / 2);
        let cell_y = 2 + (PRESETS_SIZE_Y + 2) * (i % 2);
        if mouse.x >= cell_x as f32
          && mouse.x <= (cell_x + PRESETS_SIZE_X) as f32
          && mouse.y >= cell_y as f32
          && mouse.y <= (cell_y + PRESETS_SIZE_Y) as f32
        {
          mouse_in_cell = i + 8;
          break;
        }
      }
    }

    if is_mouse_button_pressed(MouseButton::Left) && mouse_in_cell >= 0 {
      if mouse_in_cell < 8 {
        rule ^= 1 << mouse_in_cell;
      } else {
        rule = preset_values[mouse_in_cell as usize - 8];
      }

      image_clear_background(&mut image, colors::RAYWHITE);
      image_draw_pixel(&mut image, IMAGE_WIDTH / 2, 0, colors::BLACK);
      line = 1;
    }

    if line < IMAGE_HEIGHT {
      let mut i = 0;
      while i < LINES_UPDATED_PER_FRAME && line + i < IMAGE_HEIGHT {
        compute_line(&mut image, line + i, rule);
        i += 1;
      }
      line += LINES_UPDATED_PER_FRAME;

      update_texture(texture, image.data as *mut Color);
    }

    begin_drawing();
    clear_background(colors::RAYWHITE);

    draw_texture(texture, 0, SCREEN_HEIGHT - IMAGE_HEIGHT, colors::WHITE);

    for i in 0..preset_values.len() as i32 {
      draw_text(
        &format!("{}", preset_values[i as usize]),
        8 + (PRESETS_SIZE_X + 2) * (i / 2),
        4 + (PRESETS_SIZE_Y + 2) * (i % 2),
        20,
        colors::GRAY,
      );
      draw_rectangle_lines(
        4 + (PRESETS_SIZE_X + 2) * (i / 2),
        2 + (PRESETS_SIZE_Y + 2) * (i % 2),
        PRESETS_SIZE_X,
        PRESETS_SIZE_Y,
        colors::BLUE,
      );

      if mouse_in_cell == i + 8 {
        draw_rectangle_lines_ex(
          Rectangle::new(
            (2 + (PRESETS_SIZE_X + 2) * (i / 2)) as f32,
            ((PRESETS_SIZE_Y + 2) * (i % 2)) as f32,
            (PRESETS_SIZE_X + 4) as f32,
            (PRESETS_SIZE_Y + 4) as f32,
          ),
          3.0,
          colors::RED,
        );
      }
    }

    for i in 0..8 {
      for j in 0..3 {
        draw_rectangle_lines(
          DRAW_RULE_START_X - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING * j,
          DRAW_RULE_START_Y,
          DRAW_RULE_SIZE,
          DRAW_RULE_SIZE,
          colors::GRAY,
        );
        if (i & (4 >> j)) != 0 {
          draw_rectangle(
            DRAW_RULE_START_X + 2 - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING * j,
            DRAW_RULE_START_Y + 2,
            DRAW_RULE_INNER_SIZE,
            DRAW_RULE_INNER_SIZE,
            colors::BLACK,
          );
        }
      }

      draw_rectangle_lines(
        DRAW_RULE_START_X - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING,
        DRAW_RULE_START_Y + DRAW_RULE_SPACING,
        DRAW_RULE_SIZE,
        DRAW_RULE_SIZE,
        colors::BLUE,
      );
      if (rule & (1 << i)) != 0 {
        draw_rectangle(
          DRAW_RULE_START_X + 2 - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING,
          DRAW_RULE_START_Y + 2 + DRAW_RULE_SPACING,
          DRAW_RULE_INNER_SIZE,
          DRAW_RULE_INNER_SIZE,
          colors::BLACK,
        );
      }

      if mouse_in_cell == i {
        draw_rectangle_lines_ex(
          Rectangle::new(
            (DRAW_RULE_START_X - DRAW_RULE_GROUP_SPACING * i + DRAW_RULE_SPACING - 2) as f32,
            (DRAW_RULE_START_Y + DRAW_RULE_SPACING - 2) as f32,
            (DRAW_RULE_SIZE + 4) as f32,
            (DRAW_RULE_SIZE + 4) as f32,
          ),
          3.0,
          colors::RED,
        );
      }
    }

    draw_text(
      &format!("RULE: {}", rule),
      DRAW_RULE_START_X + DRAW_RULE_SPACING * 4,
      DRAW_RULE_START_Y + 1,
      30,
      colors::GRAY,
    );

    end_drawing();
  }

  unload_image(image);
  unload_texture(texture);

  close_window();
}
